use actix_web::{web, App, HttpRequest, HttpServer, Responder};

pub mod proto;
pub mod app;

async fn greet(req: HttpRequest) -> impl Responder {
    // let name = req.match_info().get("name").unwrap_or("World");
    // format!("Hello {}!", &name)
    let balance = app::pool::PoolsProvider::get_pool_test().await.unwrap();
    format!("{}", balance)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}