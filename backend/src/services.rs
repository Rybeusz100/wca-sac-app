use actix_files::NamedFile;
use actix_web::{get, web, Responder};
use log::{error, info};

use crate::{graph_type_validator::GraphTypeValidator, wca_sac::generate_graph};

#[get("/graph/{graph_type}")]
async fn get_graph(
    path: web::Path<String>,
    validator: web::Data<GraphTypeValidator>,
) -> impl Responder {
    let graph_type = path.into_inner();

    if !validator.validate(&graph_type) {
        return Err(actix_web::error::ErrorNotFound("Invalid graph type"));
    }

    let file_path = format!("../WCA_SAC/SAC_graph_{}.png", graph_type);

    // TODO cache
    if let Ok(file) = NamedFile::open_async(&file_path).await {
        Ok(file)
    } else {
        info!("Generating graph for {}", graph_type);
        generate_graph(&graph_type).await.unwrap_or_else(|e| {
            error!("Failed to generate graph: {}", e);
        });
        match NamedFile::open_async(file_path).await {
            Ok(file) => Ok(file),
            Err(_) => Err(actix_web::error::ErrorInternalServerError(
                "Failed to generate graph",
            )),
        }
    }
}

#[get("/events")]
async fn get_events() -> impl Responder {
    NamedFile::open_async("./assets/events.json").await
}

#[get("/continents")]
async fn get_continents() -> impl Responder {
    NamedFile::open_async("./assets/continents.json").await
}
