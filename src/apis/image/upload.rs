use crate::app_state::AppState;
use crate::utils::errors::AppError;
use crate::utils::response::ApiResponse;
use axum::extract::{Multipart, State};
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

pub async fn upload_image(
    State(_state): State<AppState>,
    mut multipart: Multipart,
) -> Result<ApiResponse<String>, AppError> {
    let image_location = get_image_location().await?;

    if let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?
    {
        let content_type = field
            .content_type()
            .ok_or_else(|| AppError::BadRequest("Missing content type".to_string()))?;

        if !content_type.starts_with("image/") {
            return Err(AppError::BadRequest(format!(
                "Invalid file type: {}",
                content_type
            )));
        }

        let file_extension = content_type.split('/').nth(1).unwrap_or("png");
        let file_name = format!("{}.{}", Uuid::new_v4(), file_extension);
        let image_full_path = Path::new(&image_location).join(&file_name);

        let bytes = field
            .bytes()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        fs::write(&image_full_path, bytes)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(ApiResponse::success(
            image_full_path.to_string_lossy().into_owned(),
        ))
    } else {
        Err(AppError::BadRequest("No file uploaded".to_string()))
    }
}

async fn get_image_location() -> Result<String, AppError> {
    let location = "./image_upload";
    if !Path::new(location).exists() {
        fs::create_dir_all(location).await.map_err(|e| {
            AppError::InternalServerError(format!("Failed to create directory: {}", e))
        })?;
    }
    Ok(location.to_string())
}
