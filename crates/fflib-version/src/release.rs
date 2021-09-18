use crate::error::{Error, Result};
use crate::iter::Iter;
use crate::token;
use crate::version::Version;
use proc_macro::Group;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Release {
    pub major: u16,
    pub minor: Option<u16>,
    pub patch: Option<u16>,
}

pub fn parse(paren: Group, iter: Iter) -> Result<Release> {
    try_parse(iter).map_err(|_| {
        Error::group(
            paren,
            "expected library version release number, like 58.29.100",
        )
    })
}

fn try_parse(iter: Iter) -> Result<Release, ()> {
    let major_minor = token::parse_literal(iter).map_err(drop)?;
    let string = major_minor.to_string();

    let parts: Vec<_> = string.split(".").collect();

    let major: u16 = parts[0].parse().map_err(drop)?;

    let minor: Option<u16> = if parts.len() == 2 {
        Some(parts[1].parse().map_err(drop)?)
    } else {
        None
    };

    let patch = if token::parse_optional_punct(iter, '.').is_some() {
        let int = token::parse_literal(iter).map_err(drop)?;
        Some(int.to_string().parse().map_err(drop)?)
    } else {
        None
    };

    Ok(Release {
        major,
        minor,
        patch,
    })
}

impl PartialEq<Release> for Version {
    fn eq(&self, other: &Release) -> bool {
        self.major == other.major
            && other.minor.map_or(true, |m| m == self.minor)
            && other.patch.map_or(true, |p| p == self.patch)
    }
}

impl PartialOrd<Release> for Version {
    fn partial_cmp(&self, other: &Release) -> Option<std::cmp::Ordering> {
        let version = (self.major, self.minor, self.patch);
        let release = (
            other.major,
            other.minor.unwrap_or(0),
            other.patch.unwrap_or(0),
        );

        Some(version.cmp(&release))
    }
}
