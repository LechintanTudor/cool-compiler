use cool_collections::SmallString;
use derive_more::{Constructor, Display, Error, From};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::fmt;
use std::num::IntErrorKind;
use std::str::FromStr;

#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Debug,
    Display,
    Constructor,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[display("{major}.{minor}.{patch}")]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(version_str: &str) -> Result<Self, Self::Err> {
        let mut char_iter = version_str.chars();
        let mut number_buffer: SmallString = SmallString::new();

        for c in char_iter.by_ref() {
            if c.is_numeric() {
                number_buffer.push(c);
            } else if c == '.' {
                if char_iter.as_str().is_empty() {
                    return Err(ParseVersionError::UnexpectedEof);
                }

                break;
            } else {
                return Err(ParseVersionError::UnexpectedChar { char: c });
            }
        }

        let major = parse_version_number(&number_buffer, VersionNumber::Major)?;

        if char_iter.as_str().is_empty() {
            return Ok(Version {
                major,
                ..Default::default()
            });
        }

        number_buffer.clear();

        for c in char_iter.by_ref() {
            if c.is_numeric() {
                number_buffer.push(c);
            } else if c == '.' {
                if char_iter.as_str().is_empty() {
                    return Err(ParseVersionError::UnexpectedEof);
                }

                break;
            } else {
                return Err(ParseVersionError::UnexpectedChar { char: c });
            }
        }

        let minor = parse_version_number(&number_buffer, VersionNumber::Minor)?;

        if char_iter.as_str().is_empty() {
            return Ok(Version {
                major,
                minor,
                ..Default::default()
            });
        }

        number_buffer.clear();

        for c in char_iter {
            if !c.is_numeric() {
                return Err(ParseVersionError::UnexpectedChar { char: c });
            }

            number_buffer.push(c);
        }

        let patch = parse_version_number(&number_buffer, VersionNumber::Patch)?;

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

fn parse_version_number(
    number_str: &str,
    number: VersionNumber,
) -> Result<u32, VersionNumberError> {
    if number_str.len() != 1 && number_str.starts_with('0') {
        return Err(VersionNumberError {
            number,
            kind: VersionNumberErrorKind::InvalidLeadingZero,
        });
    }

    number_str.parse::<u32>().map_err(|error| {
        let kind = match error.kind() {
            IntErrorKind::Empty => VersionNumberErrorKind::Empty,
            IntErrorKind::PosOverflow => VersionNumberErrorKind::Overflow,
            _ => VersionNumberErrorKind::Unknown,
        };

        VersionNumberError { number, kind }
    })
}

#[derive(Clone, PartialEq, Eq, From, Error, Debug)]
pub enum ParseVersionError {
    UnexpectedChar { char: char },
    UnexpectedEof,
    InvalidVersionNumber(#[error(source)] VersionNumberError),
}

impl fmt::Display for ParseVersionError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse package version")
    }
}

#[derive(Clone, PartialEq, Eq, Error, Debug)]
pub struct VersionNumberError {
    pub number: VersionNumber,
    pub kind: VersionNumberErrorKind,
}

impl fmt::Display for VersionNumberError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse {} version number", self.number)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub enum VersionNumber {
    #[display("major")]
    Major,

    #[display("minor")]
    Minor,

    #[display("patch")]
    Patch,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Display)]
pub enum VersionNumberErrorKind {
    #[display("unknown error")]
    Unknown,

    #[display("empty")]
    Empty,

    #[display("invalid leading zero")]
    InvalidLeadingZero,

    #[display("overflow")]
    Overflow,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_ok() {
        assert!(Version::from_str("1.2.3").is_ok_and(|v| v == Version::new(1, 2, 3)));
        assert!(Version::from_str("1.2").is_ok_and(|v| v == Version::new(1, 2, 0)));
        assert!(Version::from_str("1").is_ok_and(|v| v == Version::new(1, 0, 0)));
    }

    #[test]
    fn to_str() {
        assert_eq!(Version::new(1, 2, 3).to_string(), "1.2.3");
        assert_eq!(Version::new(1, 2, 0).to_string(), "1.2.0");
        assert_eq!(Version::new(1, 0, 0).to_string(), "1.0.0");
    }
}
