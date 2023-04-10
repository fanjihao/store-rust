use ntex::web::{ self, middleware, App, HttpServer };
use std::{ env, sync::{ Arc, Mutex }};
use sqlx::{ postgres::PgPoolOptions, Pool, Postgres };

// 引入模块
mod errors;
mod foods;
mod models;

use foods::view;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

#[ntex::main]
async fn main () {
    dotenvy::dotenv().ok();

    // log
    env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    let db_url: String = env::var("DATABASE_URL").expect("Please set DATABASE_URL");

    // state
    let app_state: Arc<AppState> = Arc::new(AppState {
        db_pool: PgPoolOptions::new().max_connections(10)
        .connect(&db_url).await.unwrap(),
    });

    HttpServer::new(move || {
        App::new().state(Arc::clone(&app_state)).wrap(middleware::Logger::default())
        .service(index)
        .service(view::get_food_list)
        .service(view::upload_pic)
    }).bind("0.0.0.0:8080")
    .unwrap().run().await.unwrap();
}

#[web::get("/")]
async fn index() -> String {
    "Hello, fan".into()
}
// #[web::post("/upload")]
// async fn upload(multipart) -> Result<(), String> {
//     if let Some(file) = multipart.next_field().await.unwrap() {
//         //文件类型
//         let content_type = file.content_type().unwrap().to_string();
 
//         //校验是否为图片(出于安全考虑)
//         if content_type.starts_with("image/") {
//             //根据文件类型生成随机文件名(出于安全考虑)
//             let rnd = (random::<f32>() * 1000000000 as f32) as i32;
//             //提取"/"的index位置
//             let index = content_type
//                 .find("/")
//                 .map(|i| i)
//                 .unwrap_or(usize::max_value());
//             //文件扩展名
//             let mut ext_name = "xxx";
//             if index != usize::max_value() {
//                 ext_name = &content_type[index + 1..];
//             }
//             //最终保存在服务器上的文件名
//             let save_filename = format!("{}/{}.{}", SAVE_FILE_BASE_PATH, rnd, ext_name);
 
//             //文件内容
//             let data = file.bytes().await.unwrap();
 
//             //辅助日志
//             println!("filename:{},content_type:{}", save_filename, content_type);
 
//             //保存上传的文件
//             tokio::fs::write(&save_filename, &data)
//                 .await
//                 .map_err(|err| err.to_string())?;
 
//             //上传成功后，显示上传后的图片
//             return redirect(format!("/show_image/{}.{}", rnd, ext_name)).await;
//         }
//     }
 
//     //正常情况，走不到这里来
//     println!("{}", "没有上传文件或文件格式不对");
 
//     //当上传的文件类型不对时，下面的重定向有时候会失败(感觉是axum的bug)
//     return redirect(format!("/upload")).await;
// }