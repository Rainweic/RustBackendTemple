use crate::app_state::AppState;
use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use std::fs;
use std::path::Path;

fn get_image_location() -> String {
    let location = "./image_upload";
    if !Path::new(location).exists() {
        fs::create_dir_all(location).expect(format!("mkdir {} failed!", location).as_str());
    }
    location.to_string()
}

pub async fn upload_image(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<StatusCode, String> {
    let image_location = get_image_location();
    let image_full_path = Path::new(&image_location).join("test.png");

    if let Some(file) = multipart.next_field().await.unwrap() {
        let content_type = file.content_type().unwrap();
        if content_type.starts_with("image/") {
            let bytes = file.bytes().await.unwrap();
            tokio::fs::write(image_full_path, bytes)
                .await
                .expect("TODO: panic message");
        } else {
            return Err(format!("file type is {:?}", content_type));
        }
    }

    Ok(StatusCode::OK)
}
