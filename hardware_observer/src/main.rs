#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]

pub mod logger;
pub mod networking;
pub mod rpc;
pub mod web;

use embassy_executor::Spawner;
use log::*;
use networking::init_networking;
use rpc::init_rpc;
use web::init_web;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    init_rpc(p.USB, spawner).await;

    let stack = init_networking(
        spawner, p.PIN_23, p.PIN_25, p.PIO0, p.PIN_24, p.PIN_29, p.DMA_CH0,
    )
    .await;

    init_web(spawner, stack).await;

    info!("everything initted");

    core::future::pending::<()>().await;
}
