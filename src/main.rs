#![no_std]
#![no_main]

mod gp2d12;
mod hardware;
mod model;
mod state;

use hardware::Hardware;
use model::{Config, Event, Model};

use cortex_m::{asm, peripheral::NVIC};
use cortex_m_rt::entry;
use panic_halt as _;
use state::State;
use stm32f3_discovery::{
    stm32f3xx_hal::{
        pac::{interrupt, Interrupt},
        timer,
    },
    switch_hal::OutputSwitch,
};

#[entry]
fn main() -> ! {
    initialize();
    State::with(update_distance);

    loop {
        asm::wfi();
    }
}

fn initialize() {
    let mut hardware = Hardware::take().unwrap();

    let config = Config {
        min_distance: hardware.distance_sensor.min_distance(),
        max_distance: hardware.distance_sensor.max_distance(),
        num_leds: hardware.leds.iter_mut().len() as u32,
    };

    let model = Model::new(config);

    State::initialize(hardware, model);
    State::with(start_timer);
}

fn start_timer(state: &mut State) {
    state.hardware.timer.listen(timer::Event::Update);

    unsafe {
        NVIC::unmask(Interrupt::TIM2);
    }
}

#[interrupt]
fn TIM2() {
    State::with(|state| {
        state.hardware.timer.clear_update_interrupt_flag();
        update_distance(state);
    });
}

fn update_distance(state: &mut State) {
    let distance = state.hardware.distance_sensor.read();
    state.model.update(Event::DistanceUpdated(distance));
    update_leds(state)
}

fn update_leds(state: &mut State) {
    let max_led = state.model.num_leds_on();

    for (index, led) in state.hardware.leds.iter_mut().enumerate() {
        if (index as u32) < max_led {
            led.on().unwrap();
        } else {
            led.off().unwrap();
        }
    }
}
