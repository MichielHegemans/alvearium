use chrono::{DateTime, Utc};
use core::fmt::Formatter;
use core::result::Result;
use serde::de::{Deserializer, Error, Visitor};
use std::str::FromStr;
use serde::Deserialize;
use crate::crypto::FromWif;
use crate::crypto::public_key::PublicKey;

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

fn deserialize_public_key<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    struct PublicKeyVisitor;

    impl<'de> Visitor<'de> for PublicKeyVisitor {
        type Value = PublicKey;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a public key in WIF format")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
            PublicKey::from_wif(v).map_err(|_| E::custom("Error building public key from WIF"))
        }
    }

    deserializer.deserialize_any(PublicKeyVisitor)
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserialize_public_key(deserializer)
    }
}
