#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

mod food;
mod game;
mod snake;
mod timer;

use core::{alloc::Layout, panic::PanicInfo};

use game::Game;
use timer::Timer;
use uefi::{
    proto::{console::gop::GraphicsOutput, rng::Rng},
    table::{Boot, SystemTable},
    Handle, Status,
};

#[no_mangle]
fn efi_main(_: Handle, system_table: SystemTable<Boot>) -> Status {
    unsafe {
        uefi::alloc::init(system_table.boot_services());
    };

    let boot_services = system_table.boot_services();

    let gop = unsafe {
        &mut *boot_services
            .locate_protocol::<GraphicsOutput>()
            .unwrap()
            .get()
    };
    let mode = gop
        .modes()
        .max_by(|x, y| {
            let (width_x, heigth_x) = x.info().resolution();
            let (width_y, heigth_y) = y.info().resolution();

            (width_x * heigth_x).cmp(&(width_y * heigth_y))
        })
        .unwrap();
    gop.set_mode(&mode).unwrap();

    let rng = unsafe { &mut *boot_services.locate_protocol::<Rng>().unwrap().get() };

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

#[alloc_error_handler]
fn alloc_error(_: Layout) -> ! {
    panic!("Alloc error");
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
