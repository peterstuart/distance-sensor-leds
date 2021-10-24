use crate::gp2d12::Gp2d12;
use stm32f3_discovery::{
    leds::Leds,
    stm32f3xx_hal::{
        adc::{self, Adc},
        delay::Delay,
        gpio::{Analog, Gpioe, Pin, U},
        pac::{self, ADC3},
        prelude::*,
    },
};

pub struct Init {
    pub delay: Delay,
    pub leds: Leds,
    pub distance_sensor: Gp2d12<Pin<Gpioe, U<7_u8>, Analog>, Adc<ADC3>>,
}

impl Init {
    pub fn take() -> Option<Self> {
        let mut peripherals = pac::Peripherals::take()?;
        let mut reset_and_clock_control = peripherals.RCC.constrain();

        let core_peripherals = cortex_m::Peripherals::take()?;
        let mut flash = peripherals.FLASH.constrain();
        let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
        let delay = Delay::new(core_peripherals.SYST, clocks);

        let mut gpioe = peripherals.GPIOE.split(&mut reset_and_clock_control.ahb);
        let leds = Leds::new(
            gpioe.pe8,
            gpioe.pe9,
            gpioe.pe10,
            gpioe.pe11,
            gpioe.pe12,
            gpioe.pe13,
            gpioe.pe14,
            gpioe.pe15,
            &mut gpioe.moder,
            &mut gpioe.otyper,
        );

        let adc3 = adc::Adc::adc3(
            peripherals.ADC3,
            &mut peripherals.ADC3_4,
            &mut reset_and_clock_control.ahb,
            adc::CkMode::default(),
            clocks,
        );
        let pe7 = gpioe.pe7.into_analog(&mut gpioe.moder, &mut gpioe.pupdr);
        let distance_sensor = Gp2d12::new(pe7, adc3);

        Some(Self {
            delay,
            leds,
            distance_sensor,
        })
    }
}
