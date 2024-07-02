use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::anyhow;
use log::{debug, info};
use tokio::process::Command;

pub struct WcaSac {
    in_progress: Arc<Mutex<Vec<String>>>,
    generated_graphs: Arc<Mutex<Vec<String>>>,
}

impl WcaSac {
    pub fn new() -> Self {
        Self {
            in_progress: Arc::new(Mutex::new(vec![])),
            generated_graphs: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn request_graph(&self, graph_type: &str) -> anyhow::Result<()> {
        debug!("Requesting graph for {}", graph_type);

        let is_in_progress = self
            .in_progress
            .lock()
            .unwrap()
            .contains(&graph_type.to_string());
        let is_generated = self
            .generated_graphs
            .lock()
            .unwrap()
            .contains(&graph_type.to_string());

        if is_generated {
            debug!("Graph for {} is already generated", graph_type);
            Ok(())
        } else if !is_in_progress {
            debug!("Graph for {} is not generated, generating...", graph_type);
            self.generate_graph(graph_type).await
        } else {
            debug!("Graph for {} is in progress, waiting...", graph_type);
            tokio::time::sleep(Duration::from_secs(1)).await;
            Box::pin(self.request_graph(graph_type)).await
        }
    }

    async fn generate_graph(&self, graph_type: &str) -> anyhow::Result<()> {
        info!("Generating graph for {}", graph_type);
        self.in_progress
            .lock()
            .unwrap()
            .push(graph_type.to_string());

        let output = Command::new("cmd")
            .arg("/C")
            .arg("python")
            .arg("create_graph.py")
            .arg(graph_type)
            .current_dir("../WCA_SAC")
            .output()
            .await?;

        self.in_progress.lock().unwrap().retain(|g| g != graph_type);

        if output.status.success() {
            self.generated_graphs
                .lock()
                .unwrap()
                .push(graph_type.to_string());
            Ok(())
        } else {
            Err(anyhow!(
                "Failed to generate graph: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}
