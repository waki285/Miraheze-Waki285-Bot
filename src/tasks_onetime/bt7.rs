#![allow(unused)]
use mwbot::SaveOptions;
use regex::Regex;
use serde_json::Value;

use crate::util::summary;
/* 
pub async fn bt7(bot: &mwbot::Bot) -> Result<(), Box<dyn std::error::Error>> {
    let list = include_str!("list.txt");

    let list = list.lines().collect::<Vec<_>>();

    for title in list {
        let title = title.trim();
        let result: Result<Value, _> = bot
            .api()
            .post_with_token("csrf", &[
                ("action", "delete"),
                ("title", &title),
                ("reason", "Mass-deleting templates ([[m:User:Waki285-Bot/tasks/BT7|Requested]])"),
                ("formatversion", "2"),
                ("watchlist", "nochange"),
                ("bot", "1"),
            ])
            .await;
        println!("Deleted {}: {:?}", title, result);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }

    Ok(())
}

    */