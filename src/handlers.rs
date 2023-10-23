use actix_multipart::{Field, Multipart};
use actix_web::{error::Result, http::header::DispositionType, web::Data, HttpRequest};
use anyhow::{Context, Error};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use serde::Serialize;
use upload_service::core::{repository::Repository, service::Service, store::Store};

#[derive(Debug, Serialize)]
struct UploadResult {
    ids: Vec<String>,
}

async fn upload<R, S, TK>(
    req: HttpRequest,
    service: Data<Service<R, S, String>>,
    mut form: Multipart,
) -> Result<UploadResult>
where
    R: Repository<String> + Clone,
    S: Store<Stream = Field> + Clone,
{
    if let Some(hv) = req.headers().get("X-User-ID") {
        if let Ok(uid) = hv.to_str() {
            let mut ids = Vec::new();
            while let Some(field) = form.next().await {
                if let Ok(field) = field {
                    if let Some(filename) = field.content_disposition().clone().get_filename() {
                        if let Ok(id) = service
                            .upload(field, filename, uid.to_owned(), Some(1024 * 1024))
                            .await
                        {
                            ids.push(id);
                        }
                    }
                }
            }
            return Ok(UploadResult { ids });
        }
    }
    Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
}
