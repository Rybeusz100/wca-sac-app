use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::{error, info};
use tokio_cron_scheduler::{Job, JobScheduler};

mod services;
mod wca_export;
mod wca_sac;

async fn wca_export_job() {
    if let Err(e) = wca_export::download_and_unzip("../WCA_SAC/data").await {
        error!("Failed to download and unzip WCA export: {}", e);
    } else {
        let entries = tokio::fs::read_dir("../WCA_SAC").await;
        if let Ok(mut entries) = entries {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".csv") || file_name.ends_with(".png") {
                        info!("Removing file: {}", file_name);
                        tokio::fs::remove_file(entry.path())
                            .await
                            .unwrap_or_else(|e| {
                                error!("Failed to remove file {}: {}", file_name, e);
                            });
                    }
                }
            }
        }
        info!("Removed old CSV and PNG files");
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

    HttpServer::new(|| {
        App::new()
            .service(services::get_graph)
            .service(services::get_events)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
