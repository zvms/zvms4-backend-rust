#[cfg(test)]
mod tests {
    use crate::{
        database,
        models::{
            activities::{Activity, ActivityStatus, ActivityType, SpecialActivityCategory},
            groups::GroupPermission,
        },
        routers::activities::{self, read::ReadActivityQuery},
        utils::jwt::{TokenType, UserData},
    };
    use axum::{extract::{Path, Query}, response::IntoResponse, Extension, Json};
    use bson::oid::ObjectId;
    use std::{str::FromStr, sync::Arc, time::SystemTime};
    use tokio::sync::Mutex;
    #[tokio::test]
    async fn get_activities() {
        let id = ObjectId::new().to_hex();
        let token = UserData {
            id: id.clone(),
            perms: vec![GroupPermission::Student, GroupPermission::Admin],
            term: TokenType::LongTerm,
        };
        let client = database::create_client().await;
        if let Err(_) = client {
            assert!(false);
        }
        let client = client.unwrap();
        let extension = Arc::new(Mutex::new(client));
        let extension = Extension(extension);
        let result = activities::read::read_all(
            extension,
            token,
            Query(ReadActivityQuery {
                page: Some(1),
                perpage: Some(10),
                query: Some("".to_string()),
            }),
        )
        .await;
        let result = result.into_response();
        assert!(result.status().is_success());
    }
    #[tokio::test]
    async fn create_activity() {
        let activity_id = ObjectId::new();
        let activity = Activity {
            _id: activity_id.clone(),
            activity_type: ActivityType::Special,
            name: "测试".to_string(),
            description: Some(
                "该义工为单元测试时自动创建，若出现长久放置请联系开发者。".to_string(),
            ),
            creator: ObjectId::from_str("65e6fa210edc81d012ec45e3").unwrap(),
            status: ActivityStatus::Pending,
            duration: Some(1.0),
            date: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            created_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            members: Some(vec![]),
            location: Some("测试".to_string()),
            category: Some(SpecialActivityCategory::Other),
        };
        println!("{:?}", activity);
        let id = ObjectId::new().to_hex();
        let token = UserData {
            id: id.clone(),
            perms: vec![GroupPermission::Student, GroupPermission::Admin],
            term: TokenType::LongTerm,
        };
        let client = database::create_client().await;
        if let Err(_) = client {
            assert!(false);
        }
        let client = client.unwrap();
        let extension = Arc::new(Mutex::new(client));
        let extension = Extension(extension);
        let result =
            activities::insert::insert_activity(extension.clone(), token.clone(), Query(Some(false)), Json(activity))
                .await;
        let result = result.into_response();
        assert!(result.status().is_success());
        let activity_read = activities::read::read_one(
            extension.clone(),
            token.clone(),
            Path(activity_id.clone().to_string()),
        )
        .await;
        let activity_read = activity_read.into_response();
        assert!(activity_read.status().is_success());
        let update_name = activities::update::UpdateActivityName {
            name: "测试 修改名称".to_string(),
        };
        let result = activities::update::update_activity_name(
            extension.clone(),
            token.clone(),
            Path(activity_id.clone().to_string()),
            Json(update_name),
        )
        .await;
        let result = result.into_response();
        println!("{:?}", result);
        assert!(result.status().is_success());
        let activity = activities::read::read_one(
            extension.clone(),
            token.clone(),
            Path(activity_id.clone().to_string()),
        )
        .await;
        let activity = activity.into_response();
        assert!(activity.status().is_success());
        let update_description = activities::update::UpdateActivityDescription {
            description: "测试 修改描述".to_string(),
        };
        let result = activities::update::update_activity_description(
            extension.clone(),
            token.clone(),
            Path(activity_id.clone().to_string()),
            Json(update_description),
        )
        .await;
        let result = result.into_response();
        assert!(result.status().is_success());
        let result = activities::remove::remove_activity(
            extension,
            token,
            Path(activity_id.clone().to_string()),
        )
        .await;
        let result = result.into_response();
        assert!(result.status().is_success());
    }
}
