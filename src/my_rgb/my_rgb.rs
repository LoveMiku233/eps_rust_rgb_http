use anyhow::Result;
use esp_idf_hal::{
    gpio::OutputPin, rmt::{TxRmtDriver, RmtChannel, PinState, config::TransmitConfig, Pulse, FixedLengthSignal}, peripheral::Peripheral    
};
use std::time::Duration;
use esp_idf_sys::imaxdiv_t;
pub use rgb::RGB8;

// WS2812
pub struct WS2812RMT<'a> {
    tx_rtm_driver: TxRmtDriver<'a>
} 

impl<'d> WS2812RMT<'d> {
    pub fn new(
        led: impl Peripheral<P = impl OutputPin> + 'd,
        channel: impl Peripheral<P = impl RmtChannel> + 'd
    ) -> Result<Self> {
        let config = TransmitConfig::new().clock_divider(2);
        let tx = TxRmtDriver::new(channel, led, &config).unwrap();
        Ok(Self { tx_rtm_driver: tx })
    }

    pub fn set_pixel(&mut self, rgb: RGB8) -> Result<()> {
        let color: u32 = ((rgb.g as u32) << 16) | ((rgb.r as u32) << 8) | (rgb.b as u32 );
        let ticks_hz = self.tx_rtm_driver.counter_clock()?;
        let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(350))?;
        let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(800))?;
        let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(700))?;
        let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(600))?;
        let mut signal = FixedLengthSignal::<24>::new();
        for i in (0..24).rev() {
            let p = 2_u32.pow(i);
            let bit = p & color != 0;
            let (high_pulse, low_pulse) = if bit {(t1h, t1l)} else {(t0h, t0l)};
            signal.set(23 - i as usize, &(high_pulse, low_pulse))?;
        }
        self.tx_rtm_driver.start_blocking(&signal)?;
        Ok(())
    }
}