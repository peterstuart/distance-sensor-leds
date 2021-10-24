use adc_interpolator::AdcInterpolator;
use stm32f3_discovery::stm32f3xx_hal::hal::adc::{Channel, OneShot};

const fn pair(voltage: u32, distance: u32) -> (u32, u32) {
    adc_interpolator::pair(3300, 12, voltage, distance)
}

const INTERPOLATOR: AdcInterpolator<18> = AdcInterpolator::new([
    pair(420, 80),
    pair(450, 75),
    pair(480, 70),
    pair(510, 65),
    pair(540, 60),
    pair(580, 55),
    pair(620, 50),
    pair(680, 45),
    pair(760, 40),
    pair(850, 35),
    pair(975, 30),
    pair(1140, 28),
    pair(1380, 20),
    pair(1520, 18),
    pair(1660, 16),
    pair(1860, 14),
    pair(2125, 12),
    pair(2450, 10),
]);

#[derive(Debug)]
pub struct Gp2d12<Pin, Adc> {
    pin: Pin,
    adc: Adc,
    adc_interpolator: AdcInterpolator<18>,
}

impl<Pin, Adc> Gp2d12<Pin, Adc> {
    pub fn new<ADC>(pin: Pin, adc: Adc) -> Self
    where
        Pin: Channel<ADC>,
        Adc: OneShot<ADC, u16, Pin>,
    {
        Self {
            pin,
            adc,
            adc_interpolator: INTERPOLATOR,
        }
    }

    pub fn read<ADC>(&mut self) -> Option<u32>
    where
        Pin: Channel<ADC>,
        Adc: OneShot<ADC, u16, Pin>,
    {
        let voltage = self.adc.read(&mut self.pin).ok().unwrap();
        self.adc_interpolator.value(voltage as u32)
    }

    pub fn min_distance(&self) -> u32 {
        self.adc_interpolator.min_value()
    }

    pub fn max_distance(&self) -> u32 {
        self.adc_interpolator.max_value()
    }
}
