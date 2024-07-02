use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use graph_type_validator::GraphTypeValidator;
use log::{error, info};
use tokio_cron_scheduler::{Job, JobScheduler};
use wca_sac::WcaSac;

mod graph_type_validator;
mod services;
mod utils;
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
    pretty_env_logger::init_timed();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .unwrap();

    wca_export_job().await;

    let job_scheduler = JobScheduler::new().await.unwrap();
    job_scheduler
        .add(Job::new_async("0 0 0 * * Tue", |_, _| Box::pin(wca_export_job())).unwrap())
        .await
        .unwrap();
    job_scheduler.start().await.unwrap();

    let validator = web::Data::new(GraphTypeValidator::new());
    let wca_sac_instance = web::Data::new(WcaSac::new());

    HttpServer::new(move || {
        App::new()
            // TODO remember to change it
            .wrap(Cors::permissive())
            .app_data(validator.clone())
            .app_data(wca_sac_instance.clone())
            .service(services::get_graph)
            .service(services::get_events)
            .service(services::get_continents)
            .service(services::get_countries)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
