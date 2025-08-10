use std::sync::Arc;
use chrono::{NaiveDateTime, TimeZone, Utc};
use mwbot::SaveOptions;
use visdom::Vis;

use crate::util::summary;

fn format_duration(seconds: i64) -> String {
    let hours = seconds / 3600;
    if hours < 1 {
        format!("<1 hour")
    } else if hours < 24 {
        format!("{} hour{}", hours, if hours > 1 { "s" } else { "" })
    } else {
        let days = hours / 24;
        let remaining_hours = hours % 24;
        if remaining_hours == 0 {
            format!("{} day{}", days, if days > 1 { "s" } else { "" })
        } else {
            format!("{} day{} {} hour{}", days, if days > 1 { "s" } else { "" }, remaining_hours, if remaining_hours > 1 { "s" } else { "" })
        }
    }
}


pub async fn get_rw_backlog(bot: &Arc<mwbot::Bot>) -> Result<(), anyhow::Error> {
    log::debug!("Updating RWQ backlog duration");

    let client = reqwest::Client::new();
    let html = client.get("https://meta.miraheze.org/wiki/Special:RequestWikiQueue?uselang=en&limit=999")
        .header("User-Agent", "Waki285-Bot")
        .send()
        .await?
        .text()
        .await?;

    dbg!(&html);

    let vis = Vis::load(&html).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let els = vis.find("td[class=TablePager_col_cw_timestamp]")
        .map(|_i, x| x.text());

    let datetimes = els.iter()
        .map(|x| Utc.from_utc_datetime(&NaiveDateTime::parse_from_str(x, "%H:%M, %d %B %Y").unwrap()))
        .collect::<Vec<_>>();

    let page = bot.page("User:Waki285-Bot/RWQ_backlog/backlog")?;


    if datetimes.is_empty() {
        tokio::spawn(async move {
            page.save("<1 hour (clean!)", &SaveOptions::summary(&summary("Updating RWQ backlog duration"))).await.unwrap();
        });
        return Ok(());
    }

    let mid = datetimes.len() / 2;
    let mid = datetimes[mid];

    let elapsed_time = Utc::now() - mid;

    let doubled = elapsed_time.num_seconds() as f64 * 2.0_f64.min(datetimes.len() as f64 / 10.0_f64);

    let mut duration = format_duration(doubled.round() as i64);
    let duration2 = duration.clone();

    duration.push_str(" (");
    duration.push_str(&datetimes.len().to_string());
    duration.push_str(&format!(" request{})", if datetimes.len() > 1 { "s" } else { "" }));

    tokio::spawn(async move {
        page.save(duration, &SaveOptions::summary(&summary(&format!("Updating RWQ backlog duration ({})", duration2)))).await.unwrap();
    });

    Ok(())
}
