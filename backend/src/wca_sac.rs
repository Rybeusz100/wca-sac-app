use anyhow::anyhow;
use tokio::process::Command;

pub async fn generate_graph(graph_type: &str) -> anyhow::Result<()> {
    let output = Command::new("cmd")
        .arg("/C")
        .arg("python")
        .arg("create_graph.py")
        .arg(graph_type)
        .current_dir("../WCA_SAC")
        .output()
        .await?;

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "Failed to generate graph: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
