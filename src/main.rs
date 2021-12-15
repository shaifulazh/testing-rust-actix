use actix_web::{web, App, HttpRequest, HttpServer, Responder, Result};
use serde::Serialize;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}
async fn hello() -> Result<impl Responder> {
    let name = "sdfasdfasdf";
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}