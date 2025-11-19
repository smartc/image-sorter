use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::fs;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct AppState {
    base_path: String,
}

#[derive(Serialize)]
struct ImageInfo {
    filename: String,
    current_folder: String,
    unsorted_count: usize,
    clear_count: usize,
    cloudy_count: usize,
    skip_count: usize,
}

#[derive(Deserialize)]
struct MoveImageRequest {
    filename: String,
    source_folder: String,
    target_folder: String,
}

#[derive(Deserialize)]
struct FolderQuery {
    folder: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        base_path: "samples".to_string(),
    };

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/current", get(get_current_image))
        .route("/api/move", post(move_image))
        .route("/image/:filename", get(serve_image))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

async fn get_current_image(
    State(state): State<Arc<AppState>>,
    Query(query): Query<FolderQuery>,
) -> Result<Json<ImageInfo>, StatusCode> {
    let current_folder = query.folder.unwrap_or_else(|| "Unsorted".to_string());

    let unsorted_path = format!("{}/Unsorted", state.base_path);
    let clear_path = format!("{}/Clear", state.base_path);
    let cloudy_path = format!("{}/Cloudy", state.base_path);
    let skip_path = format!("{}/Skip", state.base_path);

    // Get counts for all directories
    let unsorted_count = count_images(&unsorted_path).await;
    let clear_count = count_images(&clear_path).await;
    let cloudy_count = count_images(&cloudy_path).await;
    let skip_count = count_images(&skip_path).await;

    // Get first image from the current folder
    let current_path = format!("{}/{}", state.base_path, current_folder);
    let mut entries = match fs::read_dir(&current_path).await {
        Ok(entries) => entries,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mut filename = String::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        if let Ok(file_type) = entry.file_type().await {
            if file_type.is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    if is_image_file(name) {
                        filename = name.to_string();
                        break;
                    }
                }
            }
        }
    }

    Ok(Json(ImageInfo {
        filename,
        current_folder,
        unsorted_count,
        clear_count,
        cloudy_count,
        skip_count,
    }))
}

async fn move_image(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<MoveImageRequest>,
) -> Result<StatusCode, StatusCode> {
    let source = format!("{}/{}/{}", state.base_path, payload.source_folder, payload.filename);
    let destination = format!("{}/{}/{}", state.base_path, payload.target_folder, payload.filename);

    match fs::rename(&source, &destination).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            eprintln!("Error moving file: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn serve_image(
    State(state): State<Arc<AppState>>,
    Path(filename): Path<String>,
    Query(query): Query<FolderQuery>,
) -> Result<Response, StatusCode> {
    let folder = query.folder.unwrap_or_else(|| "Unsorted".to_string());
    let path = format!("{}/{}/{}", state.base_path, folder, filename);

    let content = match fs::read(&path).await {
        Ok(content) => content,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    let mime_type = if filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
        "image/jpeg"
    } else if filename.ends_with(".png") {
        "image/png"
    } else if filename.ends_with(".gif") {
        "image/gif"
    } else if filename.ends_with(".webp") {
        "image/webp"
    } else {
        "application/octet-stream"
    };

    Ok(([(header::CONTENT_TYPE, mime_type)], content).into_response())
}

async fn count_images(path: &str) -> usize {
    let mut count = 0;
    if let Ok(mut entries) = fs::read_dir(path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_file() {
                    if let Some(name) = entry.file_name().to_str() {
                        if is_image_file(name) {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

fn is_image_file(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".png")
        || lower.ends_with(".gif")
        || lower.ends_with(".webp")
}
