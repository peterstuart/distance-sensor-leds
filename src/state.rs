use crate::{hardware::Hardware, model::Model};
use core::cell::RefCell;
use cortex_m::interrupt::{self, Mutex};

static STATE: Mutex<RefCell<Option<State>>> = Mutex::new(RefCell::new(None));

pub struct State {
    pub hardware: Hardware,
    pub model: Model,
}

impl State {
    pub fn initialize(hardware: Hardware, model: Model) {
        let state = State { hardware, model };

        interrupt::free(|critical_section| {
            STATE.borrow(critical_section).replace(Some(state));
        });
    }

    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&mut State) -> R,
    {
        interrupt::free(|critical_section| {
            let state = STATE.borrow(critical_section);
            f(state.borrow_mut().as_mut().unwrap())
        })
    }
}
