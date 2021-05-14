#[cfg(feature = "std")]
use std::{io, cmp, fmt};
#[cfg(not(feature = "std"))]
use core::{cmp, fmt};

#[derive(Debug)]
pub enum DisassemblyError {
    #[cfg(feature = "std")]
    IOError(io::Error),
    InvalidHexCharacter,
    TooFewBytesForPush,
}

impl cmp::PartialEq for DisassemblyError {
    fn eq(&self, other: &Self) -> bool {
        match other {
            #[cfg(feature = "std")]
            DisassemblyError::IOError(rhs) => {
                if let DisassemblyError::IOError(lhs) = self {
                    rhs.kind() == lhs.kind()
                } else {
                    false
                }
            }
            DisassemblyError::InvalidHexCharacter => {
                if let DisassemblyError::InvalidHexCharacter = other {
                    true
                } else {
                    false
                }
            }
            DisassemblyError::TooFewBytesForPush => {
                if let DisassemblyError::TooFewBytesForPush = other {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::convert::From<io::Error> for DisassemblyError {
    fn from(err: io::Error) -> Self {
        DisassemblyError::IOError(err)
    }
}

#[cfg(feature = "std")]
impl std::convert::From<hex::FromHexError> for DisassemblyError {
    fn from(_: hex::FromHexError) -> Self {
        DisassemblyError::InvalidHexCharacter
    }
}

impl fmt::Display for DisassemblyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "std")]
            Self::IOError(err) => write!(f, "Encountered IO error: {}!", err),
            Self::InvalidHexCharacter => write!(f, "Encountered invalid hex character!"),
            Self::TooFewBytesForPush => {
                write!(f, "Too few bytes availabe to parse push operation!")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DisassemblyError {}
