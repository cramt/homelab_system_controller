use tokio::process::Command;

use crate::config::settings;

pub async fn systemd_start(service: &str) {
    systemd("start", service).await
}

pub async fn systemd_stop(service: &str) {
    systemd("stop", service).await
}

pub async fn systemd_restart(service: &str) {
    systemd("restart", service).await
}

pub async fn systemd(command: &str, service: &str) {
    Command::new(&settings().await.systemctl_path)
        .arg(command)
        .arg(format!("{service}.service"))
        .output()
        .await
        .unwrap();
}
