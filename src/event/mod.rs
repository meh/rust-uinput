use libc::c_int;

/// Trait for event type.
pub trait Kind {
	/// Get the event type.
	fn kind(&self) -> c_int;
}

/// Trait for event code.
pub trait Code {
	/// Get the event code.
	fn code(&self) -> c_int;
}

/// Trait for events that support pressing (namely keys).
pub trait Press: Kind + Code { }

/// Trait for events that support releasing (namely keys).
pub trait Release: Kind + Code { }

/// Trait for events that support absolute or relative positioning.
pub trait Position: Kind + Code { }

/// All supported events.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Event {
	All,

	/// Keyboard event.
	Keyboard(Keyboard),

	/// Controller event.
	Controller(Controller),

	/// Relative movement event.
	Relative(Relative),

	/// Absolute movement event.
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
