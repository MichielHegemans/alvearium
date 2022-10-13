use chrono::{DateTime, Utc};
use serde::Serializer;

pub fn serialize_hive_time<S>(date_time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date_time.format("%Y-%m-%dT%H:%M:%S"));
    serializer.serialize_str(&s)
}
