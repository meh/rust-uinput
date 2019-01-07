uinput
======
`/dev/uinput` high level wrapper.

Example
-------
The following example writes `hello world`.

```rust
extern crate uinput;
use uinput::event::keyboard;

use std::thread;
use std::time::Duration;

fn main() {
	let mut device = uinput::default().unwrap()
		.name("test").unwrap()
		.event(uinput::event::Keyboard::All).unwrap()
		.create().unwrap();

	thread::sleep(Duration::from_secs(1));

	device.click(&keyboard::Key::H).unwrap();
	device.click(&keyboard::Key::E).unwrap();
	device.click(&keyboard::Key::L).unwrap();
	device.click(&keyboard::Key::L).unwrap();
	device.click(&keyboard::Key::O).unwrap();
	device.click(&keyboard::Key::Space).unwrap();
	device.click(&keyboard::Key::W).unwrap();
	device.click(&keyboard::Key::O).unwrap();
	device.click(&keyboard::Key::R).unwrap();
	device.click(&keyboard::Key::L).unwrap();
	device.click(&keyboard::Key::D).unwrap();
	device.click(&keyboard::Key::Enter).unwrap();

	device.synchronize().unwrap();
}
```

Example mouse
-------------
```rust
extern crate uinput;

use std::thread;
use std::time::Duration;
use uinput::event::controller::Controller::Mouse;
use uinput::event::controller::Mouse::Left;
use uinput::event::Event::{Controller, Relative};
use uinput::event::relative::Position::{X, Y};
use uinput::event::relative::Relative::Position;

fn main() {
	let mut device = uinput::default().unwrap()
		.name("test").unwrap()
		.event(Controller(Mouse(Left))).unwrap() // It's necessary to enable any mouse button. Otherwise Relative events would not work.
		.event(Relative(Position(X))).unwrap()
		.event(Relative(Position(Y))).unwrap()
		.create().unwrap();

	for _ in 1..10 {
		thread::sleep(Duration::from_secs(1));

		device.send(X, 50).unwrap();
		device.send(Y, 50).unwrap();
		device.synchronize().unwrap();
	}
}
```
