use actix_web::{ web, App, HttpRequest, HttpServer, Responder, HttpResponse };

async fn first_api( req: HttpRequest) -> impl Responder {
    let data = req.match_info().get("name").unwrap_or("fan");
    format!("your name is {}!", &data)
    // HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(first_api)).route("/{name}", web::get().to(first_api))
    }).bind("127.0.0.1:8080")?.run().await
}
