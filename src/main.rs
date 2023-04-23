#![no_std]
#![no_main]

extern crate alloc;

mod food;
mod game;
mod snake;
mod timer;

use game::Game;
use timer::Timer;
use uefi::{
    entry,
    proto::{console::gop::GraphicsOutput, rng::Rng},
    table::{Boot, SystemTable},
    Handle, Status,
};

#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    let boot_services = system_table.boot_services();

    let gop_handle = boot_services
        .get_handle_for_protocol::<GraphicsOutput>()
        .unwrap();

    let mut gop = boot_services
        .open_protocol_exclusive::<GraphicsOutput>(gop_handle)
        .unwrap();

    let mode = gop
        .modes()
        .max_by(|x, y| {
            let (width_x, heigth_x) = x.info().resolution();
            let (width_y, heigth_y) = y.info().resolution();

            (width_x * heigth_x).cmp(&(width_y * heigth_y))
        })
        .unwrap();
    gop.set_mode(&mode).unwrap();

    let rng_handle = boot_services.get_handle_for_protocol::<Rng>().unwrap();

    let rng = boot_services
        .open_protocol_exclusive::<Rng>(rng_handle)
        .unwrap();

    let timer = Timer::new(boot_services);

    Game::new(
        gop,
        rng,
        unsafe { system_table.unsafe_clone().stdin() },
        timer,
    )
    .run();

    Status::SUCCESS
}
