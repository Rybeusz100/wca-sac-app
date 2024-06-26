use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::error;
use tokio_cron_scheduler::{Job, JobScheduler};

mod wca_export;

async fn wca_export_job() {
    if let Err(e) = wca_export::download_and_unzip("../WCA_SAC/data").await {
        error!("Failed to download and unzip WCA export: {}", e);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .unwrap();

    wca_export_job().await;
    let wca_export_job_scheduler = JobScheduler::new().await.unwrap();
    wca_export_job_scheduler
        .add(Job::new_async("0 0 0 * * Tue", |_, _| Box::pin(wca_export_job())).unwrap())
        .await
        .unwrap();
    wca_export_job_scheduler.start().await.unwrap();

    HttpServer::new(|| App::new())
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
