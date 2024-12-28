use std::net::Ipv4Addr;

use poise::{
    serenity_prelude::{CreateAttachment, CreateMessage},
    CreateReply,
};
use serde::{Deserialize, Serialize};
use tokio::join;

use crate::get_ip::get_ip;
use systemstat::{ByteSize, Duration, Filesystem, Platform, System};

#[derive(Deserialize, Serialize)]
pub struct MemoryStatus {
    total: ByteSize,
    free: ByteSize,
}

#[derive(Deserialize, Serialize)]
pub struct CPUStatus {
    user: f32,
    nice: f32,
    system: f32,
    interrupt: f32,
    idle: f32,
}

impl CPUStatus {
    fn zero() -> CPUStatus {
        CPUStatus {
            user: 0.0,
            nice: 0.0,
            system: 0.0,
            interrupt: 0.0,
            idle: 0.0,
        }
    }

    fn reduce(self, rhs: f32) -> Self {
        Self {
            user: self.user / rhs,
            nice: self.nice / rhs,
            system: self.system / rhs,
            interrupt: self.interrupt / rhs,
            idle: self.idle / rhs,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Status {
    public_ip: Option<Ipv4Addr>,
    memory: Option<MemoryStatus>,
    uptime: Option<Duration>,
    cpu_load: Option<CPUStatus>,
    mounts: Option<Vec<Filesystem>>,
}

impl Status {
    pub async fn new() -> Status {
        let sys = System::new();
        let (public_ip, cpu_load) = join!(get_ip(), cpu_load(&sys));
        Status {
            public_ip,
            memory: sys.memory().ok().map(|x| MemoryStatus {
                free: x.free,
                total: x.total,
            }),
            mounts: sys.mounts().ok(),
            uptime: sys.uptime().ok(),
            cpu_load,
        }
    }

    pub fn to_discord_reply(&self) -> CreateReply {
        CreateReply::default().attachment(self.to_discord_attachment())
    }

    pub fn to_discord_attachment(&self) -> CreateAttachment {
        match serde_yaml::to_string(self) {
            Ok(x) => CreateAttachment::bytes(x.as_bytes().to_vec(), "status.yaml"),
            Err(_) => CreateAttachment::bytes(vec![], "failed to make.yaml"),
        }
    }

    pub fn to_discord_message(&self) -> CreateMessage {
        CreateMessage::new().files(vec![self.to_discord_attachment()])
    }
}

async fn cpu_load(sys: &System) -> Option<CPUStatus> {
    let cpu = sys.cpu_load().ok()?;
    tokio::time::sleep(Duration::from_millis(100)).await;
    let raw = cpu.done().ok()?;
    let len = raw.len();
    Some(
        raw.into_iter()
            .fold(CPUStatus::zero(), |mut acc, x| {
                acc.user += x.user;
                acc.nice += x.nice;
                acc.system += x.system;
                acc.interrupt += x.interrupt;
                acc.idle += x.idle;
                acc
            })
            .reduce(len as f32),
    )
}
