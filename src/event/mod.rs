use libc::c_int;

pub trait Kind {
	fn kind(&self) -> c_int;
}

pub trait Code {
	fn code(&self) -> c_int;
}

pub trait Press: Kind + Code { }
pub trait Release: Kind + Code { }
pub trait Position: Kind + Code { }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Event {
	All,

	Keyboard(Keyboard),
	Controller(Controller),
	Relative(Relative),
	Absolute(Absolute),
}

impl Kind for Event {
	fn kind(&self) -> c_int {
		match self {
			&Event::All => unreachable!(),

			&Event::Keyboard(ref v)   => v.kind(),
			&Event::Controller(ref v) => v.kind(),
			&Event::Relative(ref v)   => v.kind(),
			&Event::Absolute(ref v)   => v.kind(),
		}
	}
}

impl Code for Event {
	fn code(&self) -> c_int {
		match self {
			&Event::All => unreachable!(),

			&Event::Keyboard(ref v)   => v.code(),
			&Event::Controller(ref v) => v.code(),
			&Event::Relative(ref v)   => v.code(),
			&Event::Absolute(ref v)   => v.code(),
		}
	}
}

pub mod keyboard;
pub use self::keyboard::Keyboard;

pub mod controller;
pub use self::controller::Controller;

pub mod relative;
pub use self::relative::Relative;

pub mod absolute;
pub use self::absolute::Absolute;
