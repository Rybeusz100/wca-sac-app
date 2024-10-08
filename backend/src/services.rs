use std::path::Path;

use actix_files::NamedFile;
use actix_web::{get, http::header, web, HttpResponse, Responder};

use crate::{graph_type_validator::GraphTypeValidator, utils::sha256, wca_sac::WcaSac};

#[get("/graph/{graph_type}")]
async fn get_graph(
    path: web::Path<String>,
    validator: web::Data<GraphTypeValidator>,
    wca_sac_instance: web::Data<WcaSac>,
) -> impl Responder {
    let graph_type = path.into_inner();
    let file_path = format!("../WCA_SAC/SAC_graph_{}.png", graph_type);
    let file_path = Path::new(&file_path);

    if validator.validate(&graph_type)
        && wca_sac_instance.request_graph(&graph_type).await.is_ok()
        && file_path.exists()
    {
        // TODO cache
        NamedFile::open_async(&file_path).await
    } else {
        NamedFile::open_async("./assets/error.gif").await
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

#[get("/countries")]
async fn get_countries(validator: web::Data<GraphTypeValidator>) -> impl Responder {
    let countries = validator.countries();
    let countries_json = serde_json::to_string(countries);

    if let Ok(countries_json) = countries_json {
        let etag = sha256(countries_json);

        HttpResponse::Ok()
            .insert_header((header::CACHE_CONTROL, "public, max-age=3600"))
            .insert_header((header::ETAG, etag))
            .json(countries)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
