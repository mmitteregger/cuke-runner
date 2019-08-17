use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::time::Duration;

pub trait SerdeNanos {
    fn encode<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer;

    fn decode<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where Self: Sized, D: Deserializer<'de>;
}

impl SerdeNanos for Duration {
    fn encode<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.as_nanos().serialize(serializer)
    }

    fn decode<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where Self: Sized, D: Deserializer<'de>,
    {
        let val = Deserialize::deserialize(deserializer)?;
        Ok(Duration::from_nanos(val))
    }
}

pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where T: SerdeNanos, S: Serializer
{
    SerdeNanos::encode(value, serializer)
}
