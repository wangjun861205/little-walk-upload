use actix_multipart::Multipart;
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, Result},
    web::{Data, Json},
    HttpRequest,
};
use futures::{StreamExt, TryStreamExt};
use serde::Serialize;
use upload_service::core::{repository::Repository, service::Service, store::Store};

#[derive(Debug, Serialize)]
pub struct UploadResult {
    ids: Vec<String>,
}

pub(crate) async fn upload<R, S>(
    req: HttpRequest,
    service: Data<Service<R, S, String>>,
    mut form: Multipart,
) -> Result<Json<UploadResult>>
where
    R: Repository<String> + Clone,
    S: Store + Clone,
{
    let hv = req
        .headers()
        .get("X-User-ID")
        .ok_or(ErrorBadRequest("Unauthorized"))?;
    let uid = hv.to_str().map_err(ErrorBadRequest)?;
    let mut ids = Vec::new();
    while let Some(field) = form.next().await {
        let field = field.map_err(ErrorInternalServerError)?;
        let disposition = field.content_disposition().clone();
        let filename = disposition
            .get_filename()
            .ok_or(ErrorBadRequest("failed to get filename"))?;
        let id = service
            .upload(
                field.map_err(|e| anyhow::Error::msg(e.to_string())),
                filename,
                uid.to_owned(),
                Some(1024 * 1024),
            )
            .await
            .map_err(ErrorInternalServerError)?;
        ids.push(id);
    }
    Ok(Json(UploadResult { ids }))
}
