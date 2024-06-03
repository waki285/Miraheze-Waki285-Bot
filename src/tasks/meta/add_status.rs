use std::sync::Arc;

use mwbot::SaveOptions;

use crate::util::{check_status, extract_sections_with_titles, summary};

pub async fn add_status(bot: &Arc<mwbot::Bot>) -> Result<(), anyhow::Error> {
    let status = check_status(bot.clone()).await;
    if !status {
        log::warn!("status is false");
        return Err(anyhow::anyhow!("status is false"));
    }

    log::debug!("add status");

    let an = bot.page("Meta:Administrators'_noticeboard");
    if let Err(e) = an {
        log::error!("Error retrieving page: {:?}", e);
        return Err(anyhow::anyhow!("Error retrieving page"));
    }
    let an = an.unwrap();
    let mut an_text = an.wikitext().await.unwrap();
    let an_sections = extract_sections_with_titles(&an_text);
    for (_title, section) in an_sections {
        if !section.to_lowercase().contains("{{status") {
            let new_section = "{{status}}\n".to_string() + &section.trim_start();
            an_text = an_text.replace(&section, &new_section);
        }
    }
    match an
        .save(&an_text, &SaveOptions::summary(&summary("add status")))
        .await
    {
        Ok(_) => {}
        Err(e) => {
            log::error!("Error saving page: {:?}", e);
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let global = bot.page("Steward_requests/Global");
    if let Err(e) = global {
        log::error!("Error retrieving page: {:?}", e);
        return Err(anyhow::anyhow!("Error retrieving page"));
    }
    let global = global.unwrap();
    let mut global_text = global.wikitext().await.unwrap();
    let global_sections = extract_sections_with_titles(&global_text);
    for (_title, section) in global_sections {
        if !section.to_lowercase().contains("{{status")
            && !section.to_lowercase().contains("{{permission")
        {
            let new_section = "{{status}}\n".to_string() + &section.trim_start();
            global_text = global_text.replace(&section, &new_section);
        }
    }
    match global
        .save(&global_text, &SaveOptions::summary(&summary("add status")))
        .await
    {
        Ok(_) => {}
        Err(e) => {
            println!("Error saving page: {:?}", e);
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let reports = bot.page("Steward_requests/Wiki_reports");
    if let Err(e) = reports {
        log::error!("Error retrieving page: {:?}", e);
        return Err(anyhow::anyhow!("Error retrieving page"));
    }
    let reports = reports.unwrap();
    let mut reports_text = reports.wikitext().await.unwrap();
    let reports_sections = extract_sections_with_titles(&reports_text);
    for (_title, section) in reports_sections {
        if !section.to_lowercase().contains("{{status") {
            let new_section = "{{status}}\n".to_string() + &section.trim_start();
            reports_text = reports_text.replace(&section, &new_section);
        }
    }
    match reports
        .save(&reports_text, &SaveOptions::summary(&summary("add status")))
        .await
    {
        Ok(_) => {}
        Err(e) => {
            println!("Error saving page: {:?}", e);
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let dc = bot.page("Steward_requests/Discussion_closure");
    if let Err(e) = dc {
        log::error!("Error retrieving page: {:?}", e);
        return Err(anyhow::anyhow!("Error retrieving page"));
    }
    let dc = dc.unwrap();
    let mut dc_text = dc.wikitext().await.unwrap();
    let dc_sections = extract_sections_with_titles(&dc_text);
    for (_title, section) in dc_sections {
        if !section.to_lowercase().contains("{{status") {
            let new_section = "{{status}}\n".to_string() + &section.trim_start();
            dc_text = dc_text.replace(&section, &new_section);
        }
    }
    match dc
        .save(&dc_text, &SaveOptions::summary(&summary("add status")))
        .await
    {
        Ok(_) => {}
        Err(e) => {
            println!("Error saving page: {:?}", e);
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let misc = bot.page("Steward_requests/Miscellaneous");
    if let Err(e) = misc {
        log::error!("Error retrieving page: {:?}", e);
        return Err(anyhow::anyhow!("Error retrieving page"));
    }
    let misc = misc.unwrap();
    let mut misc_text = misc.wikitext().await.unwrap();
    let misc_sections = extract_sections_with_titles(&misc_text);
    for (_title, section) in misc_sections {
        if !section.to_lowercase().contains("{{status")
            && !section.to_lowercase().contains("{{sn")
            && !section.to_lowercase().contains("{{permission")
        {
            let new_section = "{{status}}\n".to_string() + &section.trim_start();
            misc_text = misc_text.replace(&section, &new_section);
        }
    }
    match misc
        .save(&misc_text, &SaveOptions::summary(&summary("add status")))
        .await
    {
        Ok(_) => {}
        Err(e) => {
            log::error!("Error saving page: {:?}", e);
        }
    }

    Ok(())
}
