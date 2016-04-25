#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Id {
	pub bus:     u16,
	pub vendor:  u16,
	pub product: u16,
	pub version: u16,
}

mod builder;
pub use self::builder::Builder;

mod device;
pub use self::device::Device;

