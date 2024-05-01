#[cfg(test)]
mod tests {
    use crate::{
        database,
        models::groups::GroupPermission,
        routers::activities::{self, read::ReadActivityQuery},
        utils::jwt::{TokenType, UserData},
    };
    use axum::{extract::Query, response::IntoResponse, Extension};
    use bson::oid::ObjectId;
    use std::sync::Arc;
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
}
