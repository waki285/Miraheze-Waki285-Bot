use std::sync::Arc;

use mwbot::SaveOptions;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::util::{check_status, extract_sections_with_titles, summary};

static LOCKHIDE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\{\{\s*(?i)LockHide\s*\|\s*(?-i)(.+?)(\|hidename=1)?\s*\}\}").unwrap()
});
static MULTILOCK_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)\{\{\s*(?i)MultiLock\s*\|\s*(?-i)(.+?)(\|hidename=1)?\s*\}\}").unwrap()
});
static LUXOTOOL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\{\{\s*(?i)Luxotool\s*\|\s*(?-i)(.+?)\s*\}\}").unwrap()
});

pub async fn srg_mark_done(bot: &Arc<mwbot::Bot>) -> Result<(), anyhow::Error> {
    let status = check_status(bot.clone()).await;
    if !status {
        log::warn!("status is false");
        return Err(anyhow::anyhow!("status is false"));
    }

    log::debug!("marking done");

    let srg = bot.page("Steward_requests/Global");
    if let Err(e) = srg {
        log::error!("Error retrieving page: {:?}", e);
        return Err(anyhow::anyhow!("Error retrieving page"));
    }
    let srg = srg.unwrap();
    let mut srg_text = srg.wikitext().await.unwrap();
    let srg_sections = extract_sections_with_titles(&srg_text);

    // get sections that is in progress
    let in_progress_sections = srg_sections
        .iter()
        .filter(|(_title, section)| {
            section.to_lowercase().contains("{{status}}")
        })
        .collect::<Vec<_>>();

    // exclude unlock / unblock requests
    let in_progress_sections = in_progress_sections
        .iter()
        .filter(|(title, _section)| {
            !title.to_lowercase().contains("unlock")
                && !title.to_lowercase().contains("unblock")
        })
        .collect::<Vec<_>>();

    let mut done = 0;

    for (title, section) in in_progress_sections {
        // get target account names (ex. {{LockHide|BadUsername}})
        let lockhide_accounts = LOCKHIDE_REGEX
            .captures_iter(section)
            .map(|c| c.get(1).unwrap().as_str())
            .collect::<Vec<_>>();
        let multilock_accounts = MULTILOCK_REGEX
            .captures_iter(section)
            .map(|c| c.get(1).unwrap().as_str())
            .map(|s| s.split('|').collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>();
        let luxotool_ips = LUXOTOOL_REGEX
            .captures_iter(section)
            .map(|c| c.get(1).unwrap().as_str())
            .collect::<Vec<_>>();

        let accounts = lockhide_accounts
            .iter()
            .chain(multilock_accounts.iter())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let mut locked = 0;
        let mut blocked = 0;
        let mut did_people = vec![];

        for account in &accounts {
            let info = bot.api().get_value(&[
                ("action", "query"),
                ("format", "json"),
                ("list", "globalallusers"),
                ("agulimit", "1"),
                ("agufrom", &account),
                ("aguto", &account),
                ("aguprop", "lockinfo"),
                ("formatversion", "2")
            ])
            .await?;
            let locked_info = &info["query"]["globalallusers"].as_array().unwrap()[0];
            if locked_info["locked"].is_string() {
                locked += 1;
                let did = bot.api().get_value(&[
                    ("action", "query"),
                    ("format", "json"),
                    ("list", "logevents"),
                    ("formatversion", "2"),
                    ("leprop", "user|title|type|details"),
                    ("letype", "globalauth"),
                    ("leaction", "globalauth/setstatus"),
                    ("letitle", format!("User:{}@global", account).as_str())
                ]).await?;
                let did = &did["query"]["logevents"].as_array().unwrap();
                let user = did.iter().find_map(|e| {
                    if e["params"]["added"].as_array().unwrap().iter().any(|p| p.as_str().unwrap() == "locked") {
                        Some(e["user"].as_str().unwrap())
                    } else {
                        None
                    }
                });
                if let Some(user) = user {
                    did_people.push(user.to_string());
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        for ip in &luxotool_ips {
            let info = bot.api().get_value(&[
                ("action", "query"),
                ("format", "json"),
                ("list", "globalblocks"),
                ("bgprop", "address|by|expiry"),
                ("bgip", &ip),
                ("formatversion", "2")
            ])
            .await?;
            let blocked_info = &info["query"]["globalblocks"].as_array().unwrap();
            if !blocked_info.is_empty() {
                blocked += 1;
                did_people.push(blocked_info[0]["by"].as_str().unwrap().to_string());
            }
        }

        if locked >= accounts.len() && blocked >= luxotool_ips.len() {
            log::debug!("{}: Seems like all accounts are locked and all IPs are blocked, marking done", title);
            done += 1;
            let new_section = section.replace("{{status}}", "{{status|done}}") + format!("\n::'''Robot clerk''': {{{{done}}}} by {}. ~~~~\n", did_people.join(", ")).as_str();
            srg_text = srg_text.replace(section, &new_section);
            log::trace!("done: {:?}", did_people);
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    if done == 0 {
        log::trace!("no sections marked as done");
        return Ok(());
    }

    log::trace!("done: {}", done);

    srg.save(&srg_text, &SaveOptions::summary(&summary(&format!("Marking {} request{} as done", 
        done, if done == 1 { "" } else { "s" }
    )))).await?;

    Ok(())
}