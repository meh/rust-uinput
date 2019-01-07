//! Move the mouse courser by 10 pixels in Y and X direction, every second
extern crate uinput;

use uinput::event::Relative;
use uinput::event::relative::Position;

use std::{thread, time};

fn main() {

    let mut device = uinput::default().unwrap()
		.name("Example device").unwrap()
        .vendor(0x1234)
        .product(0x5678)
        .event(uinput::event::controller::Controller::Mouse(uinput::event::controller::Mouse::Left)).unwrap()
        .event(uinput::Event::Relative(Relative::Position(Position::X))).unwrap()
        .event(uinput::Event::Relative(Relative::Position(Position::Y))).unwrap()
        .create().unwrap();

    loop {
        println!("Move 10x10");
        device.position(&Position::X, 10).unwrap();
        device.position(&Position::Y, 10).unwrap();
        device.synchronize().unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}