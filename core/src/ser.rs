use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use crate::crypto::IntoWif;
use crate::crypto::public_key::PublicKey;

pub fn serialize_hive_time<S>(date_time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date_time.format("%Y-%m-%dT%H:%M:%S"));
    serializer.serialize_str(&s)
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_wif())
    }
}
