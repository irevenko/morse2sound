use std::{convert::TryFrom, str::FromStr};
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Unit {
    Dot,
    Dash,
    Space,
    Slash,
}

#[derive(Debug, PartialEq)]
pub struct Message(pub Vec<Unit>);

impl TryFrom<char> for Unit {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Self::Dot),
            '-' => Ok(Self::Dash),
            ' ' => Ok(Self::Space),
            '/' => Ok(Self::Slash),
            _ => Err(anyhow::anyhow!("invalid morse unit '{}'", c)),
        }
    }
}

impl FromStr for Message {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let units = s.chars().map(Unit::try_from).collect::<Result<Vec<_>>>()?;
        Ok(Self(units))
    }
}