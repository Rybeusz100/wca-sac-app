use anyhow::anyhow;
use tokio::process::Command;

pub async fn generate_graph() -> anyhow::Result<()> {
    let output = Command::new("cmd")
        .arg("/C")
        .arg("python")
        .arg("create_graph.py")
        .arg("333_A")
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
