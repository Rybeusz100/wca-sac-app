use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod wca_export;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .unwrap();

    wca_export::download_and_unzip("../WCA_SAC/data")
        .await
        .unwrap();

    HttpServer::new(|| App::new())
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
