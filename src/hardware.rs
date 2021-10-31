use crate::gp2d12::Gp2d12;
use stm32f3_discovery::{
    leds::Leds,
    stm32f3xx_hal::{
        adc::{self, Adc},
        gpio::{Analog, Gpioe, Pin, U},
        pac::{self, ADC3, TIM2},
        prelude::*,
        time::rate::Hertz,
        timer::Timer,
    },
};

pub struct Hardware {
    pub leds: Leds,
    pub distance_sensor: Gp2d12<Pin<Gpioe, U<7_u8>, Analog>, Adc<ADC3>>,
    pub timer: Timer<TIM2>,
}

impl Hardware {
    pub fn take() -> Option<Self> {
        let mut peripherals = pac::Peripherals::take()?;
        let mut rcc = peripherals.RCC.constrain();

        let mut flash = peripherals.FLASH.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut gpioe = peripherals.GPIOE.split(&mut rcc.ahb);
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
            &mut rcc.ahb,
            adc::CkMode::default(),
            clocks,
        );
        let pe7 = gpioe.pe7.into_analog(&mut gpioe.moder, &mut gpioe.pupdr);
        let distance_sensor = Gp2d12::new(pe7, adc3);

        let tim2 = peripherals.TIM2;
        let timer = Timer::tim2(tim2, Hertz::new(100), clocks, &mut rcc.apb1);

        Some(Self {
            leds,
            distance_sensor,
            timer,
        })
    }
}
