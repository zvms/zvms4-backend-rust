use std::fmt;

use bson::oid::ObjectId;
use chrono::DateTime;
use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize};

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
    pub _id: String, // ObjectId
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
    pub actioner: String, // ObjectId
    pub action: ActivityMemberStatus,
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
    pub duration: Option<f64>,
    #[serde(deserialize_with = "datetime_or_u64")]
    pub date: u64,
    #[serde(deserialize_with = "datetime_or_u64")]
    pub created_at: u64,
    #[serde(deserialize_with = "datetime_or_u64")]
    pub updated_at: u64,
    pub creator: ObjectId, // ObjectId
    pub status: ActivityStatus,
    pub members: Option<Vec<ActivityMember>>,
    pub location: Option<String>,
    pub category: Option<SpecialActivityCategory>,
}

fn datetime_or_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct DateTimeOrU64;

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

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            DateTime::parse_from_rfc3339(value)
                .map_err(de::Error::custom)
                .and_then(|dt| Ok(dt.timestamp() as u64))
        }
    }

    deserializer.deserialize_any(DateTimeOrU64)
}
