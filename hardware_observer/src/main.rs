#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]

pub mod logger;
pub mod networking;
pub mod pin_context;
pub mod rpc;
pub mod web;

use embassy_executor::Spawner;
use log::*;
use networking::init_networking;
use pin_context::PinContext;
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

    let pin_context = PinContext::new(p.ADC, p.PIN_26, p.PIN_22);

    init_web(spawner, stack, pin_context).await;

    info!("everything initted");
}
