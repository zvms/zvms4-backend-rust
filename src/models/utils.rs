use crate::utils::config::load_config_sync;
use bson::oid::ObjectId;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use serde::de::Visitor;
use serde::{de, Deserializer, Serializer};
use std::fmt;
use std::str::FromStr;

pub fn object_id_to_string<S>(value: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_hex())
}

pub fn string_to_object_id<'de, D>(deserializer: D) -> Result<ObjectId, D::Error>
where
    D: Deserializer<'de>,
{
    struct ObjectIdOrStringVisitor;

    impl<'de> Visitor<'de> for ObjectIdOrStringVisitor {
        type Value = ObjectId;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or ObjectId")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            ObjectId::from_str(v).map_err(de::Error::custom)
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            if let Some((key, value)) = map.next_entry::<String, String>()? {
                if key == "$oid" {
                    return self.visit_str(&value);
                }
            }
            Err(de::Error::custom("expected a map with a key of '$oid'"))
        }
    }

    deserializer.deserialize_any(ObjectIdOrStringVisitor)
}

pub fn datetime_or_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct DateTimeOrU64 {
        timezone: chrono::FixedOffset,
    }

    impl<'de> Visitor<'de> for DateTimeOrU64 {
        type Value = u64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a UNIX timestamp as a u64 or a datetime string")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value as u64)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
                Ok(dt.timestamp() as u64)
            } else if let Ok(naive_dt) = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S") {
                let dt = self
                    .timezone
                    .from_local_datetime(&naive_dt)
                    .single()
                    .ok_or_else(|| {
                        de::Error::custom("Invalid datetime for the specified timezone")
                    })?;
                Ok(dt.timestamp() as u64)
            } else {
                Err(de::Error::custom("Invalid datetime format"))
            }
        }
    }

    let config = load_config_sync().unwrap();
    let offset: i32 = config.timezone.parse().unwrap();
    let timezone = chrono::FixedOffset::east_opt(offset * 3600).unwrap();
    let visitor = DateTimeOrU64 { timezone };
    deserializer.deserialize_any(visitor)
}
