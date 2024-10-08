use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::anyhow;
use log::info;
use tokio::process::Command;

pub struct WcaSac {
    is_stopped: Arc<Mutex<bool>>,
    in_progress: Arc<Mutex<Vec<String>>>,
    generated_graphs: Arc<Mutex<Vec<String>>>,
}

impl WcaSac {
    pub fn new() -> Self {
        Self {
            is_stopped: Arc::new(Mutex::new(false)),
            in_progress: Arc::new(Mutex::new(vec![])),
            generated_graphs: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn stop(&self) {
        info!("Stopping WcaSac instance");
        *self.is_stopped.lock().unwrap() = true;
        self.wait_until_in_progress_empty().await
    }

    pub fn start(&self) {
        info!("Starting WcaSac instance");
        *self.is_stopped.lock().unwrap() = false
    }

    fn is_stopped(&self) -> bool {
        *self.is_stopped.lock().unwrap()
    }

    async fn wait_until_in_progress_empty(&self) {
        if self.in_progress.lock().unwrap().len() > 0 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            Box::pin(self.wait_until_in_progress_empty()).await
        }
    }

    pub async fn request_graph(&self, graph_type: &str) -> anyhow::Result<()> {
        info!("Requested graph for {}", graph_type);

        if self.is_stopped() {
            info!("Generating graphs is currently stopped");
            return Ok(());
        }

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
            info!("Graph for {} is already generated", graph_type);
            Ok(())
        } else if !is_in_progress {
            info!("Graph for {} is not generated, generating...", graph_type);
            self.generate_graph(graph_type).await
        } else {
            info!("Graph for {} is in progress, waiting...", graph_type);
            tokio::time::sleep(Duration::from_secs(1)).await;
            Box::pin(self.request_graph(graph_type)).await
        }
    }

    async fn generate_graph(&self, graph_type: &str) -> anyhow::Result<()> {
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
