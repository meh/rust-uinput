use std::ffi;
use nix;

#[cfg(feature = "udev")]
use udev;

#[derive(Debug)]
pub enum Error {
	Nix(nix::Error),
	Nul(ffi::NulError),

	#[cfg(feature = "udev")]
	Udev(udev::Error),

	NotFound,
}

impl From<ffi::NulError> for Error {
	fn from(value: ffi::NulError) -> Self {
		Error::Nul(value)
	}
}

impl From<nix::Error> for Error {
	fn from(value: nix::Error) -> Self {
		Error::Nix(value)
	}
}

#[cfg(feature = "udev")]
impl From<udev::Error> for Error {
	fn from(value: udev::Error) -> Self {
		Error::Udev(value)
	}
}
