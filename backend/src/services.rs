use actix_web::{get, web, HttpResponse, Responder};
use tokio::{fs::File, io::AsyncReadExt};

use crate::wca_sac::generate_graph;

#[get("/graph/{graph_type}")]
async fn get_graph(path: web::Path<String>) -> impl Responder {
    let graph_type = path.into_inner();

    generate_graph(&graph_type).await.unwrap();

    let mut file = File::open(format!("../WCA_SAC/SAC_graph_{}.png", graph_type))
        .await
        .unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await.unwrap();
    HttpResponse::Ok().content_type("image/png").body(buffer)
}
