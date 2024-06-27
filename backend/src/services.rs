use actix_files::NamedFile;
use actix_web::{get, web, Responder};
use log::info;

use crate::wca_sac::generate_graph;

#[get("/graph/{graph_type}")]
async fn get_graph(path: web::Path<String>) -> impl Responder {
    let graph_type = path.into_inner();
    let file_path = format!("../WCA_SAC/SAC_graph_{}.png", graph_type);

    if let Ok(file) = NamedFile::open_async(&file_path).await {
        Ok(file)
    } else {
        info!("Generating graph for {}", graph_type);
        generate_graph(&graph_type).await.unwrap();
        NamedFile::open_async(file_path).await
    }
}

#[get("/events")]
async fn get_events() -> impl Responder {
    NamedFile::open_async("./assets/events.json").await
}
