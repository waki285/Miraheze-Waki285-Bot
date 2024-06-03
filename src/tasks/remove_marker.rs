use std::sync::Arc;

use mwbot::SaveOptions;

use crate::util::{check_status, extract_sections_with_titles, summary};

pub async fn remove_marker(bot: &Arc<mwbot::Bot>) -> Result<(), anyhow::Error> {
    let status = check_status(bot.clone()).await;
    if !status {
        log::warn!("status is false");
        return Err(anyhow::anyhow!("status is false"));
    }

    // remove marker
    log::debug!("remove marker");

    let page = bot.page("Meta:Requests for permissions");
    if page.is_err() {
        println!("Error retrieving page: {:?}", page.unwrap_err());
        return Err(anyhow::anyhow!("Error retrieving page"));
    }
    let page = page.unwrap();
    let mut text = page.wikitext().await.unwrap();

    // section で分割
    let sections = extract_sections_with_titles(&text);

    for (_title, section) in sections {
        let is_finished = section.contains("The following discussion is closed. '''Please do not modify it'''. Subsequent comments should be made in a new section.") || section.contains("{{Discussion top|");
        if !is_finished {
            continue;
        }
        let replaced = section
            .replace(
                "{{marker|rfp_v}} <!-- REMOVE marker template if request ended -->\n",
                "",
            )
            .replace(
                "{{marker|rfp_d}} <!-- REMOVE marker template if request ended -->\n",
                "",
            );
        text = text.replace(&section, &replaced);
    }

    match page
        .save(&text, &SaveOptions::summary(&summary("remove marker")))
        .await
    {
        Ok(_) => {}
        Err(e) => {
            println!("Error saving page: {:?}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
