use embassy_rp::{
    adc::{Adc, AdcPin, Async, Channel, Config, InterruptHandler},
    bind_interrupts,
    gpio::{Level, Output, Pin, Pull},
    peripherals::ADC,
    Peripheral,
};
use embassy_time::Timer;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

pub struct PinContext {
    adc: Adc<'static, Async>,
    // green
    pc_led: Channel<'static>,
    // red
    pc_button: Output<'static>,
}

impl PinContext {
    pub fn new(
        adc: ADC,
        pc_led_pin: impl Peripheral<P = impl AdcPin + 'static> + 'static,
        pc_button_pin: impl Peripheral<P = impl Pin> + 'static,
    ) -> Self {
        Self {
            adc: Adc::new(adc, Irqs, Config::default()),
            pc_led: Channel::new_pin(pc_led_pin, Pull::None),
            pc_button: Output::new(pc_button_pin, Level::Low),
        }
    }

    pub async fn read_pc_led(&mut self) -> Result<u16, embassy_rp::adc::Error> {
        self.adc.read(&mut self.pc_led).await
    }

    pub async fn click_button(&mut self) {
        self.pc_button.set_high();
        Timer::after_secs(1).await;
        self.pc_button.set_low();
    }
}
