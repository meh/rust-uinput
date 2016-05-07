use libc::c_int;
use ffi::*;
use {Event};
use super::{Kind, Code};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Absolute {
	Position(Position),
	Wheel(Wheel),
	Hat(Hat),
	Digi(Digi),
	Multi(Multi),
}

impl Into<Event> for Absolute {
	fn into(self) -> Event {
		Event::Absolute(self)
	}
}

impl super::Position for Absolute { }

impl Kind for Absolute {
	fn kind(&self) -> c_int {
		EV_ABS
	}
}

impl Code for Absolute {
	fn code(&self) -> c_int {
		match self {
			&Absolute::Position(ref v) => v.code(),
			&Absolute::Wheel(ref v)    => v.code(),
			&Absolute::Hat(ref v)      => v.code(),
			&Absolute::Digi(ref v)     => v.code(),
			&Absolute::Multi(ref v)    => v.code(),
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

impl Into<Event> for Position {
	fn into(self) -> Event {
		Event::Absolute(Absolute::Position(self))
	}
}

impl super::Position for Position { }

impl Kind for Position {
	fn kind(&self) -> c_int {
		EV_ABS
	}
}

impl Code for Position {
	fn code(&self) -> c_int {
		match self {
			&Position::X  => ABS_X,
			&Position::Y  => ABS_Y,
			&Position::Z  => ABS_Z,
			&Position::RX => ABS_RX,
			&Position::RY => ABS_RY,
			&Position::RZ => ABS_RZ,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(WheelVariants))]
	pub enum Wheel {
		Throttle,
		Rudder,
		Position,
		Gas,
		Brake,
	}
}

impl Into<Event> for Wheel {
	fn into(self) -> Event {
		Event::Absolute(Absolute::Wheel(self))
	}
}

impl super::Position for Wheel { }

impl Kind for Wheel {
	fn kind(&self) -> c_int {
		EV_ABS
	}
}

impl Code for Wheel {
	fn code(&self) -> c_int {
		match self {
			&Wheel::Throttle => ABS_THROTTLE,
			&Wheel::Rudder   => ABS_RUDDER,
			&Wheel::Position => ABS_WHEEL,
			&Wheel::Gas      => ABS_GAS,
			&Wheel::Brake    => ABS_BRAKE,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(HatVariants))]
	pub enum Hat {
		X0,
		Y0,
		X1,
		Y1,
		X2,
		Y2,
		X3,
		Y3,
	}
}

impl Into<Event> for Hat {
	fn into(self) -> Event {
		Event::Absolute(Absolute::Hat(self))
	}
}

impl super::Position for Hat { }

impl Kind for Hat {
	fn kind(&self) -> c_int {
		EV_ABS
	}
}

impl Code for Hat {
	fn code(&self) -> c_int {
		match self {
			&Hat::X0 => ABS_HAT0X,
			&Hat::Y0 => ABS_HAT0Y,
			&Hat::X1 => ABS_HAT1X,
			&Hat::Y1 => ABS_HAT1Y,
			&Hat::X2 => ABS_HAT2X,
			&Hat::Y2 => ABS_HAT2Y,
			&Hat::X3 => ABS_HAT3X,
			&Hat::Y3 => ABS_HAT3Y,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(DigiVariants))]
	pub enum Digi {
		Pressure,
		Distance,
		TiltX,
		TiltY,
		ToolWidth,
		Volume,
	}
}

impl Into<Event> for Digi {
	fn into(self) -> Event {
		Event::Absolute(Absolute::Digi(self))
	}
}

impl super::Position for Digi { }

impl Kind for Digi {
	fn kind(&self) -> c_int {
		EV_ABS
	}
}

impl Code for Digi {
	fn code(&self) -> c_int {
		match self {
			&Digi::Pressure  => ABS_PRESSURE,
			&Digi::Distance  => ABS_DISTANCE,
			&Digi::TiltX     => ABS_TILT_X,
			&Digi::TiltY     => ABS_TILT_Y,
			&Digi::ToolWidth => ABS_TOOL_WIDTH,
			&Digi::Volume    => ABS_VOLUME,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(MultiVariants))]
	pub enum Multi {
		Slot,
		TouchMajor,
		TouchMinor,
		WidthMajor,
		WidthMinor,
		Orientation,
		PositionX,
		PositionY,
		ToolType,
		BlobId,
		TrackingId,
		Pressure,
		Distance,
		ToolX,
		ToolY,
	}
}

impl Into<Event> for Multi {
	fn into(self) -> Event {
		Event::Absolute(Absolute::Multi(self))
	}
}

impl super::Position for Multi { }

impl Kind for Multi {
	fn kind(&self) -> c_int {
		EV_ABS
	}
}

impl Code for Multi {
	fn code(&self) -> c_int {
		match self {
			&Multi::Slot        => ABS_MT_SLOT,
			&Multi::TouchMajor  => ABS_MT_TOUCH_MAJOR,
			&Multi::TouchMinor  => ABS_MT_TOUCH_MINOR,
			&Multi::WidthMajor  => ABS_MT_WIDTH_MAJOR,
			&Multi::WidthMinor  => ABS_MT_WIDTH_MINOR,
			&Multi::Orientation => ABS_MT_ORIENTATION,
			&Multi::PositionX   => ABS_MT_POSITION_X,
			&Multi::PositionY   => ABS_MT_POSITION_Y,
			&Multi::ToolType    => ABS_MT_TOOL_TYPE,
			&Multi::BlobId      => ABS_MT_BLOB_ID,
			&Multi::TrackingId  => ABS_MT_TRACKING_ID,
			&Multi::Pressure    => ABS_MT_PRESSURE,
			&Multi::Distance    => ABS_MT_DISTANCE,
			&Multi::ToolX       => ABS_MT_TOOL_X,
			&Multi::ToolY       => ABS_MT_TOOL_Y,
		}
	}
}
