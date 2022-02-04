use std::fmt;
use std::error;
use std::ffi;
use nix;

#[cfg(feature = "udev")]
use udev;

/// UInput error.
#[derive(Debug)]
pub enum Error {
	/// System errors.
	Nix(nix::Error),

	/// Errors with internal nulls in names.
	Nul(ffi::NulError),

	#[cfg(feature = "udev")]
	/// Errors coming from udev.
	Udev(udev::Error),

	/// The uinput file could not be found.
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

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			&Error::Nix(ref err) => write!(f, "{}", err),

			&Error::Nul(ref err) => write!(f, "{}", err),

			#[cfg(feature = "udev")]
			&Error::Udev(ref err) => write!(f, "{}", err),

			&Error::NotFound =>  write!(f, "Device not found.")
		}
	}
}

impl error::Error for Error {
}
