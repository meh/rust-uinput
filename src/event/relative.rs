use libc::c_int;
use ffi::*;
use {Event};
use super::{Kind, Code};


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Relative {
	Position(Position),
	Wheel(Wheel),
}

impl From<Relative> for Event {
	fn from(val: Relative) -> Self {
		Event::Relative(val)
	}
}

impl super::Position for Relative { }

impl Kind for Relative {
	fn kind(&self) -> c_int {
		EV_REL
	}
}

impl Code for Relative {
	fn code(&self) -> c_int {
		match *self {
			Relative::Position(ref v) => v.code(),
			Relative::Wheel(ref v)    => v.code(),
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(PositionVariants))]
	pub enum Position {
		X,
		Y,
		Z,
		RX,
		RY,
		RZ,
	}
}

impl From<Position> for Event {
	fn from(val: Position) -> Self {
		Event::Relative(Relative::Position(val))
	}
}

impl super::Position for Position { }

impl Kind for Position {
	fn kind(&self) -> c_int {
		EV_REL
	}
}

impl Code for Position {
	fn code(&self) -> c_int {
		match *self {
			Position::X  => REL_X,
			Position::Y  => REL_Y,
			Position::Z  => REL_Z,
			Position::RX => REL_RX,
			Position::RY => REL_RY,
			Position::RZ => REL_RZ,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(WheelVariants))]
	pub enum Wheel {
		Horizontal,
		Dial,
		Vertical,
	}
}

impl From<Wheel> for Event {
	fn from(val: Wheel) -> Self {
		Event::Relative(Relative::Wheel(val))
	}
}

impl super::Position for Wheel { }

impl Kind for Wheel {
	fn kind(&self) -> c_int {
		EV_REL
	}
}

impl Code for Wheel {
	fn code(&self) -> c_int {
		match *self {
			Wheel::Horizontal => REL_HWHEEL,
			Wheel::Dial       => REL_DIAL,
			Wheel::Vertical   => REL_WHEEL,
		}
	}
}
