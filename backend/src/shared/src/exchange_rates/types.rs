use std::borrow::Borrow;

use candid::CandidType;
use serde::Deserialize;
use tinystr::TinyStr16;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Ticker(pub TinyStr16);

impl Borrow<str> for Ticker {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl<T> From<T> for Ticker
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self(TinyStr16::from_str(value.as_ref()).unwrap())
    }
}

impl CandidType for Ticker {
    fn _ty() -> candid::types::Type {
        String::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        self.0.idl_serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Ticker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Ticker(
            TinyStr16::from_str(String::deserialize(deserializer)?.as_str()).unwrap(),
        ))
    }
}
