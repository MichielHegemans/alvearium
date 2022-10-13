use chrono::{DateTime, Utc};
use core::fmt::Formatter;
use core::result::Result;
use serde::de::{Deserializer, Error, Visitor};
use std::str::FromStr;

pub fn deserialize_hive_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    struct HiveTimeVisitor;

    impl<'de> Visitor<'de> for HiveTimeVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("an ISO 8601 datetime string without timezone information")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            DateTime::from_str(&format!("{}Z", v)).map_err(E::custom)
        }
    }

    deserializer.deserialize_any(HiveTimeVisitor)
}
