#![no_std]
#![no_main]

mod gp2d12;
mod init;
mod model;

use cortex_m_rt::entry;
use init::Init;
use model::{Config, Event, Model};
use panic_halt as _;
use stm32f3_discovery::{leds::Leds, stm32f3xx_hal::prelude::*, switch_hal::OutputSwitch};

#[entry]
fn main() -> ! {
    let Init {
        mut leds,
        mut delay,
        mut distance_sensor,
    } = Init::take().unwrap();

    let config = Config {
        min_distance: distance_sensor.min_distance(),
        max_distance: distance_sensor.max_distance(),
        num_leds: leds.iter_mut().len() as u32,
    };
    let mut model = Model::new(config);

    loop {
        let distance = distance_sensor.read();
        model.update(Event::DistanceUpdated(distance));
        view(&mut leds, &model);

        delay.delay_ms(25u16);
    }
}

fn view(leds: &mut Leds, model: &Model) {
    let max_led = model.num_leds_on();

    for (index, led) in leds.iter_mut().enumerate() {
        if (index as u32) < max_led {
            led.on().unwrap();
        } else {
            led.off().unwrap();
        }
    }
}
