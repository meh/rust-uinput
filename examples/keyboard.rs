extern crate uinput;
use uinput::event::keyboard;

use std::thread;
use std::time::Duration;

fn main() {
	let mut device = uinput::default().unwrap()
		.name("test").unwrap()
		.event(uinput::event::keyboard::Key::K).unwrap()
		.create().unwrap();

	thread::sleep(Duration::from_secs(1));

	device.press(keyboard::Key::K).unwrap();
	device.release(keyboard::Key::K).unwrap();
	device.synchronize().unwrap();
}
