use ntex::web::{
    self,
    types::{ Json, State, Payload },
    App, HttpResponse, HttpServer, error
};
use std::sync::{ Arc, Mutex };

use crate::{ errors::CustomError, models::foods::Foods, AppState };

use rand::prelude::random;
use tempfile::{NamedTempFile, TempPath};
use std::io::{Write, copy, SeekFrom, Seek};
use std::fs::File;

#[web::get("/foods-list")]
pub async fn get_food_list(state: State<Arc<Mutex<AppState>>>) -> Result<Json<Vec<Foods>>, CustomError> {
    let db_pool = &state.lock().unwrap().db_pool;

    let foods = sqlx::query!("SELECT * FROM foods")
        .fetch_all(db_pool)
        .await?
        .iter().
        map(|i| Foods {
            id: Some(i.id as u32),
            name: i.name.clone(),
            photo: i.photo.clone().unwrap_or_default(),
            upload_date: i.upload_date.clone().unwrap_or_default(),
        })
        .collect::<Vec<Foods>>();
    Ok(Json(foods))
}
const SAVE_FILE_BASE_PATH: &str = "/Users/jimmy/Downloads/upload";

#[web::post("/upload")]
pub async fn upload_pic(mut payload: Payload) -> Result<HttpResponse, error::PayloadError> {
    let mut temp_file = File::create("/Users/admin/rust-upload/photo").unwrap();
    while let Some(chunk) = ntex::util::stream_recv(&mut payload).await {
        let data = chunk.unwrap();
        println!("data: {:?}", &data);
        temp_file.write(&data).unwrap();
    }
    temp_file.seek(std::io::SeekFrom::Start(0)).unwrap();
    let temp_file_path = temp_file.path().to_string_lossy().to_string();
    Ok(HttpResponse::Ok().body(format!("File uploaded to {}", temp_file_path.to_string_lossy()))
}