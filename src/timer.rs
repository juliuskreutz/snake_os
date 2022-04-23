use uefi::{
    prelude::BootServices,
    table::boot::{EventType, TimerTrigger, Tpl},
    Event,
};

use crate::game;

pub struct Timer<'a> {
    event: Event,
    boot_services: &'a BootServices,
}

impl<'a> Timer<'a> {
    pub fn new(boot_services: &'a BootServices) -> Self {
        let event = unsafe {
            boot_services
                .create_event(EventType::TIMER, Tpl::APPLICATION, None, None)
                .unwrap()
        };
        boot_services
            .set_timer(&event, TimerTrigger::Periodic(game::TIME_STEP))
            .unwrap();

        Self {
            event,
            boot_services,
        }
    }

    pub fn ready(&self) -> bool {
        self.boot_services
            .check_event(unsafe { self.event.unsafe_clone() })
            .unwrap()
    }
}
