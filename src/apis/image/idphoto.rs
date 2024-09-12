use crate::app_state::AppState;
use crate::utils::errors::AppError;
use crate::utils::response::ApiResponse;
use axum::{extract::Multipart, extract::State};
use base64;
use log::log;
use phf::phf_set;
use reqwest::multipart as reqwest_multipart;
use serde_json::{json, Value};
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

static ALLOWED_FIELDS: phf::Set<&'static str> = phf_set! {
    "input_image",
    "mode_option",
    "size_list_option",
    "color_option",
    "render_option",
    "image_kb_options",
    "custom_color_R",
    "custom_color_G",
    "custom_color_B",
    "custom_size_height",
    "custom_size_width",
    "custom_image_kb",
    "language",
    "matting_model_option",
    "watermark_option",
    "watermark_text",
    "watermark_text_color",
    "watermark_text_size",
    "watermark_text_opacity",
    "watermark_text_angle",
    "watermark_text_space",
    "face_detect_option",
    "head_measure_ratio",
    "top_distance_max",
    "top_distance_min"
};

async fn save_image(
    image_data: &str,
    suffix: &str,
    uuid: &Uuid,
    image_location: &str,
) -> Result<String, AppError> {
    let decoded =
        base64::decode(image_data).map_err(|e| AppError::InternalServerError(e.to_string()))?;
    let file_name = format!("{}_{}.png", uuid, suffix);
    let image_full_path = Path::new(image_location).join(&file_name);
    fs::write(&image_full_path, decoded)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(file_name)
}

pub async fn upload_image(mut multipart: Multipart) -> Result<ApiResponse<Value>, AppError> {
    let mut form = reqwest_multipart::Form::new();
    let mut input_image = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?
    {
        let name = field.name().unwrap_or("").to_string();

        if !ALLOWED_FIELDS.contains(name.as_str()) {
            return Err(AppError::BadRequest(format!("Invalid field: {}", name)));
        }

        if name == "input_image" {
            let content_type = field
                .content_type()
                .ok_or_else(|| AppError::BadRequest("Missing content type".to_string()))?;
            if !content_type.starts_with("image/") {
                return Err(AppError::BadRequest(format!(
                    "Invalid file type: {}",
                    content_type
                )));
            }
            let data = field
                .bytes()
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;
            input_image = Some(data);
        } else {
            let value = field
                .text()
                .await
                .map_err(|e| AppError::BadRequest(e.to_string()))?;
            form = form.text(name, value);
        }
    }

    let input_image =
        input_image.ok_or_else(|| AppError::BadRequest("No input image provided".to_string()))?;
    form = form.part(
        "input_image",
        reqwest_multipart::Part::bytes(input_image.to_vec()).file_name("image.jpg"),
    );

    // 调用 HivisionIDPhotos API
    let client = reqwest::Client::new();
    let response = client
        .post("http://hivision_idphotos_api:7890/process_idphoto")
        .multipart(form)
        .send()
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    log::info!("send request to hivision_idphotos_api: {:?}", response);

    if !response.status().is_success() {
        return Err(AppError::InternalServerError(
            "Failed to process image".to_string(),
        ));
    }

    let result: Value = response
        .json()
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    // 保存所有返回的图片
    let image_location = get_image_location().await?;
    let uuid = Uuid::new_v4();

    let standard_image = save_image(
        result["standard_image"].as_str().unwrap_or("standard_image"),
        "standard",
        &uuid,
        &image_location,
    )
    .await?;
    let hd_image = save_image(
        result["hd_image"].as_str().unwrap_or("hd_image"),
        "hd",
        &uuid,
        &image_location,
    )
    .await?;
    let standard_png = save_image(
        result["standard_png"].as_str().unwrap_or("standard_png"),
        "standard_png",
        &uuid,
        &image_location,
    )
    .await?;
    let hd_png = save_image(
        result["hd_png"].as_str().unwrap_or("hd_png"),
        "hd_png",
        &uuid,
        &image_location,
    )
    .await?;
    let layout_image = match result["layout_image"].as_str() {
        Some(data) => Some(save_image(data, "layout", &uuid, &image_location).await?),
        None => None,
    };

    let saved_images = json!({
        "standard_image": standard_image,
        "hd_image": hd_image,
        "standard_png": standard_png,
        "hd_png": hd_png,
        "layout_image": layout_image,
        "notification": result["notification"],
        "download_path": result["download_path"]
    });

    Ok(ApiResponse::success(saved_images))
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
