use crate::utils::config::load_config_sync;
use bson::oid::ObjectId;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{fmt, str::FromStr};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityType {
    Specified,
    Social,
    Scale,
    Special,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityStatus {
    Effective,
    Pending,
    Refused,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityMemberStatus {
    Effective,
    Pending,
    Refused,
    Rejected,
    Draft,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityMode {
    OnCampus,
    OffCampus,
    SocialPractice,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum SpecialActivityCategory {
    Prize,
    Import,
    Club,
    Deduction,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActivityMember {
    #[serde(serialize_with = "objectid_to_string", deserialize_with = "string_to_objectid")]
    pub _id: ObjectId, // ObjectId
    pub status: ActivityMemberStatus,
    pub impression: Option<String>,
    pub duration: f64,
    pub mode: ActivityMode,
    pub history: Option<Vec<ActivityMemberHistory>>,
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActivityMemberHistory {
    pub impression: String,
    pub duration: f64,
    pub time: String,
    #[serde(serialize_with = "objectid_to_string", deserialize_with = "string_to_objectid")]
    pub actioner: ObjectId, // ObjectId
    pub action: ActivityMemberStatus,
}

pub fn objectid_to_string<S>(value: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_hex())
}

pub fn string_to_objectid<'de, D>(deserializer: D) -> Result<ObjectId, D::Error>
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub name: String,
    pub description: Option<String>,
    #[serde(deserialize_with = "datetime_or_u64")]
    pub date: u64,
    #[serde(deserialize_with = "datetime_or_u64")]
    pub created_at: u64,
    #[serde(deserialize_with = "datetime_or_u64")]
    pub updated_at: u64,
    #[serde(serialize_with = "objectid_to_string", deserialize_with = "string_to_objectid")]
    pub creator: ObjectId, // ObjectId
    pub status: ActivityStatus,
    pub members: Option<Vec<ActivityMember>>,
    pub location: Option<String>,
    pub category: Option<SpecialActivityCategory>,
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
