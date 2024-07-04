#![allow(unused)]
use mwbot::SaveOptions;
use regex::Regex;

use crate::util::summary;

pub async fn bt1(bot: &mwbot::Bot) -> Result<(), Box<dyn std::error::Error>> {
    let list = include_str!("./list.txt");

    let list = list.lines().collect::<Vec<_>>();

    let regex = Regex::new(r"(?i)\[\[Category:SRE[ _]guidelines[_ ]and[ _]guides\]\]")?;

    for title in list {
        let title = title.trim();
        let page = bot.page(title)?;
        let text = page.wikitext().await?;

        let text = regex.replace_all(&text, "[[Category:Technology guidelines and guides]]");

        //dbg!(&text);

        page.save(text.to_string(), &SaveOptions::summary(&summary(&format!("Onetime Task ([[User:Waki285-Bot/tasks/BT1]]) Replace redirect category"))).mark_as_bot(false)).await.ok();

        println!("Edited: {}", title);

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }

    Ok(())
}