use crate::app_state::AppState;
use crate::utils::errors::AppError;
use crate::utils::response::ApiResponse;
use axum::{extract::Multipart, extract::State};
use reqwest::multipart as reqwest_multipart;
use serde_json::Value;
use uuid::Uuid;
use std::path::Path;
use tokio::fs;
use base64;

pub async fn upload_image(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<ApiResponse<Value>, AppError> {
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

        let bytes = field
            .bytes()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        // 准备调用 HivisionIDPhotos API
        let client = reqwest::Client::new();
        let form = reqwest_multipart::Form::new()
            .part("input_image", reqwest_multipart::Part::bytes(bytes.to_vec()).file_name("image.jpg"))
            .text("height", "413")
            .text("width", "295")
            .text("human_matting_model", "hivision_modnet")
            .text("face_detect_model", "mtcnn")
            .text("hd", "true")
            .text("head_measure_ratio", "0.2")
            .text("head_height_ratio", "0.45")
            .text("top_distance_max", "0.12")
            .text("top_distance_min", "0.10");

        let response = client
            .post("http://hivision_idphotos_api:7890/idphoto")  // 使用 Docker 服务名称
            .multipart(form)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::InternalServerError("Failed to process image".to_string()));
        }

        let result: Value = response.json().await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        // 如果需要，你可以在这里保存处理后的图片
        if let Some(standard_image) = result["image_base64_standard"].as_str() {
            let decoded = base64::decode(standard_image)
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;
            let file_name = format!("{}.png", Uuid::new_v4());
            let image_location = get_image_location().await?;
            let image_full_path = Path::new(&image_location).join(&file_name);
            fs::write(&image_full_path, decoded)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        }

        Ok(ApiResponse::success(result))
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
