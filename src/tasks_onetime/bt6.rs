use std::collections::HashMap;

use itertools::Itertools;
//use mwbot::SaveOptions;
use serde_json::Value;

#[derive(Clone, Debug)]
struct Page {
    title: String,
    text: String,
    contributor: String,
    timestamp: String,
    summary: String,
    model: String,
}

pub async fn bt6(bot: &mwbot::Bot) -> Result<(), Box<dyn std::error::Error>> {
    let xml = include_str!("../../xmls/zhbackroom.xml");
    let xml = roxmltree::Document::parse(xml)?;

    let mut namespaces = HashMap::new();
    for nss in xml.descendants().filter(|tag| tag.has_tag_name("namespace")) {
        let ns = nss.text()
            .unwrap_or("")
            .to_owned();
        let ns_key: i32 = nss.attribute("key").unwrap().parse()?;
        namespaces.insert(ns_key, ns);
    }
    dbg!(&namespaces);

    let mut pages = Vec::new();
    for page in xml.descendants().filter(|tag| tag.has_tag_name("page")) {
        let revision = page
            .descendants()
            .filter(|tag| tag.has_tag_name("revision"))
            .sorted_by(|a, b| {
                let a = a
                    .descendants()
                    .find(|tag| tag.has_tag_name("id"))
                    .unwrap();
                let a: u32 = a.text()
                    .unwrap()
                    .parse().unwrap();
                let b = b
                    .descendants()
                    .find(|tag| tag.has_tag_name("id"))
                    .unwrap();
                let b: u32 = b.text()
                    .unwrap()
                    .parse().unwrap();
                Ord::cmp(&b, &a)
            })
            .next()
            .unwrap();
        //dbg!(revision);
        let title = page
            .descendants()
            .find(|tag| tag.has_tag_name("title"))
            .unwrap();
        let title = title.text()
            .unwrap()
            .to_owned();
        /*let ns = page
            .descendants()
            .find(|tag| tag.has_tag_name("ns"))
            .unwrap();
        let ns = ns.text()
            .unwrap()
            .parse::<i32>()?;
        if ns == 3000 {
            println!("Skipping: {}", title);
        }*/
        let text = revision
            .descendants()
            .find(|tag| tag.has_tag_name("text"))
            .unwrap();
        let text = text.text()
            .unwrap_or("")
            .to_owned();
        let contributor = revision
            .descendants()
            .find(|tag| tag.has_tag_name("username"))
            .unwrap_or_else(|| page.descendants().find(|tag| tag.has_tag_name("ip")).unwrap());

        let contributor = contributor.text()
            .unwrap_or("")
            .to_owned();

        let summary = revision
            .descendants()
            .find(|tag| tag.has_tag_name("comment"))
            .map(|f| f.text().unwrap().to_owned())
            .unwrap_or("".to_owned());

        let model = revision
            .descendants()
            .find(|tag| tag.has_tag_name("model"))
            .unwrap();

        let model = model.text()
            .unwrap()
            .to_owned();

        let timestamp = revision
            .descendants()
            .find(|tag| tag.has_tag_name("timestamp"))
            .unwrap();

        let timestamp = timestamp.text()
            .unwrap()
            .to_owned();

        pages.push(Page {
            title,
            text,
            contributor,
            summary,
            timestamp,
            model,
        });
    }

    let mut errors = Vec::new();

    for (i, page) in pages.iter().enumerate() {
        let page = page.clone();
        let title = page.title;
        let text = page.text;
        let contributor = page.contributor;
        let timestamp = page.timestamp;
        let summary = page.summary;
        let model = page.model;

        let result: Result<Value, _> = bot
            .api()
            .post_with_token("csrf", &[
                ("action", "edit"),
                ("title", &title),
                ("text", &text),
                ("summary", format!("[[m:User:Waki285-Bot/tasks/BT6|Import]]: {} ({}): {}", contributor, timestamp, summary).as_str()),
                ("contentmodel", &model),
                ("bot", "1"),
                ("createonly", "1"),
                ("watchlist", "nochange"),
                ("formatversion", "2"),
            ])
            .await;

        match result {
            Ok(_) => {
                println!("Edited: {} ({}/{})", title, i + 1, pages.len());
            }
            Err(e) => {
                println!("Error: {:?} ({}/{})", e, i + 1, pages.len());
                errors.push((title, e));
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    if !errors.is_empty() {
        println!("Errors:");
        for (title, e) in errors {
            println!("{}: {:?}", title, e);
        }
    }

    Ok(())
}

