use crate::{
    models::{activities::Activity, users::User},
    routers::users::time::UserActivityTime,
};
use bson::{doc, from_document};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database};
use polars::{
    df,
    frame::DataFrame,
    io::{csv::CsvWriter, SerWriter},
    prelude::NamedFrom,
    series::Series,
};
use std::{fs::File, sync::Arc};
use tokio::sync::Mutex;

pub async fn export_to_dataframe(db: Arc<Mutex<Database>>) -> Result<DataFrame, String> {
    let db = db.lock().await;
    let mut df = df!(
        "_id" => &["".to_string()],
        "id" => &["0".to_string()],
        "name" => &["Example".to_string()],
        "class" => &["".to_string()],
        "on_campus" => &[0.0],
        "off_campus" => &[0.0],
        "social_practice" => &[0.0],
        "total" => &[0.0]
    )
    .unwrap();

    println!("Start to export data");

    let users_collection: Collection<User> = db.collection("users");
    let activities_collection: Collection<Activity> = db.collection("activities");

    let mut users = users_collection.find(doc! {}, None).await.unwrap();

    let mut count = 0;

    while let Some(doc) = users.try_next().await.unwrap() {
        count += 1;
        if count % 100 == 99 {
            return Ok(df);
        }
        println!("Processing {}'s data", doc.name);
        let pipeline = vec![
            doc! {
                "$match": {
                    "$or": [
                        { "members._id": doc._id.clone() },
                        { "members._id": doc._id.to_hex() }
                    ]
                }
            },
            doc! {
                "$unwind": "$members"
            },
            doc! {
                "$match": {
                    "$or": [
                        { "members._id": doc._id.clone() },
                        { "members._id": doc._id.to_hex() }
                    ]
                }
            },
            doc! {
                "$group": {
                    "_id": "$members.mode",
                    "totalDuration": { "$sum": "$members.duration" }
                }
            },
            doc! {
                "$group": {
                    "_id": null,
                    "on_campus": {
                        "$sum": {
                            "$cond": [{ "$eq": ["$_id", "on-campus"] }, "$totalDuration", 0.0]
                        }
                    },
                    "off_campus": {
                        "$sum": {
                            "$cond": [{ "$eq": ["$_id", "off-campus"] }, "$totalDuration", 0.0]
                        }
                    },
                    "social_practice": {
                        "$sum": {
                            "$cond": [{ "$eq": ["$_id", "social-practice"] }, "$totalDuration", 0.0]
                        }
                    },
                    "total": { "$sum": "$totalDuration" }
                }
            },
            doc! {
                "$project": {
                    "_id": 0,
                    "on_campus": 1,
                    "off_campus": 1,
                    "social_practice": 1,
                    "total": 1
                }
            },
        ];
        let cursor = activities_collection.aggregate(pipeline, None).await;
        println!("Got cursor");
        if let Err(_) = cursor {
            return Err("Failed to get cursor".to_string());
        }
        let mut cursor = cursor.unwrap();
        println!("Unwrapped cursor");
        let result = cursor.try_next().await;
        if let Err(_) = result {
            return Err("Failed to get result".to_string());
        }
        println!("Unwrapped cursor");
        let result = result.unwrap();
        if let None = result {
            continue;
        }
        println!("Unwrapped cursor");
        let result = result.unwrap();
        println!("Got result {:?}", result);
        let result: UserActivityTime = from_document(result).unwrap();
        println!("Got result");
        let extend = DataFrame::new(vec![
            Series::new("_id", vec![doc._id.clone().to_hex()]),
            Series::new("id", vec![doc.id.clone()]),
            Series::new("name", vec![doc.name.clone()]),
            Series::new("class", vec!["".to_string()]),
            Series::new("on_campus", vec![result.on_campus]),
            Series::new("off_campus", vec![result.off_campus]),
            Series::new("social_practice", vec![result.social_practice]),
            Series::new("total", vec![result.total]),
        ]);
        if let Err(_) = extend {
            return Err("Failed to create DataFrame".to_string());
        }
        println!("Extended {}'s data", doc.name);
        let extend = extend.unwrap();
        df.extend(&extend).unwrap();
    }
    Ok(df)
}

pub async fn save_to_csv(mut df: DataFrame, mut target: &File) -> Result<(), String> {
    let writer = CsvWriter::new(&mut target).finish(&mut df);
    println!("Finished writing");
    if let Err(_) = writer {
        return Err("Failed to write DataFrame".to_string());
    }
    Ok(())
}
