use std::{
    collections::{HashMap, HashSet, VecDeque}, path::Path, sync::Arc
};

use mwbot::SaveOptions;
use once_cell::sync::Lazy;
use regex::Regex;

fn summary(s: &str) -> String {
    format!("Bot: {}", s)
}

const IMPLICIT_GROUPS: [&str; 3] = ["*", "user", "autoconfirmed"];
const TEMPORAL_GROUPS: [&str; 4] = ["checkuser", "suppress", "electionadmin", "flood"];
const GLOBAL_TEMPORAL_GROUPS: [&str; 1] = ["global-flood"];

const REWRITE_GROUPS: Lazy<HashMap<&str, &str>> = Lazy::new(|| [
    ("trustandsafety", "trust-and-safety"),
].into_iter().collect());

static STATUS_PAGE: &str = "User:Waki285-Bot/status";

async fn check_status(bot: Arc<mwbot::Bot>) -> bool {
    let page = bot.page(STATUS_PAGE).unwrap();
    let text = page.wikitext().await.unwrap();
    text.contains("true")
}

fn extract_sections_with_titles(text: &str) -> Vec<(String, String)> {
    let header_regex = Regex::new(r"^== ?([^=]+) ?==$").unwrap();
    let mut sections = Vec::new();
    let mut current_section = VecDeque::new();
    let mut current_title = String::new();
    let mut in_section = false;

    for line in text.lines() {
        if let Some(caps) = header_regex.captures(line) {
            if in_section {
                if !current_section.is_empty() {
                    let section: String = current_section
                        .iter()
                        .map(|s: &String| (*s).to_string() + "\n")
                        .collect();
                    sections.push((current_title.clone(), section.trim_end().to_string()));
                    current_section.clear();
                }
            } else {
                in_section = true;
            }
            current_title = caps.get(1).unwrap().as_str().trim().to_string();
        } else if in_section {
            current_section.push_back(line.to_string());
        }
    }

    if !current_section.is_empty() {
        let section: String = current_section
            .iter()
            .map(|s| (*s).to_string() + "\n")
            .collect();
        sections.push((current_title, section.trim_end().to_string()));
    }

    sections
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = Arc::new(
        mwbot::Bot::from_path(Path::new("./.config/mwbot.toml"))
            .await
            .unwrap(),
    );

    let bot_clone = Arc::clone(&bot);
    tokio::spawn(async move {
        let bot = Arc::clone(&bot_clone);
        loop {
            let status = check_status(bot.clone()).await;
            if !status {
                println!("status is false");
                // 1時間
                tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;

                continue;
            }

            println!("cleaning sandbox");

            let page = bot.page("Meta:Sandbox").unwrap();
            page.save(
                "{{Please leave this line alone (sandbox heading)}}",
                &SaveOptions::summary(&summary("cleaning sandbox")),
            )
            .await
            .unwrap();

            // 12時間
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60 * 12)).await;
        }
    });

    let bot_clone2 = Arc::clone(&bot);
    tokio::spawn(async move {
        let bot_clone2 = Arc::clone(&bot_clone2);
        loop {
            let bot = bot_clone2.clone();

            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

            let status = check_status(bot.clone()).await;
            if !status {
                println!("status is false");
                // 1時間
                tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;

                continue;
            }
            println!("othergroups");
            let data = bot
                .api()
                .get_value(&[
                    ("action", "query"),
                    ("list", "allusers"),
                    (
                        "augroup",
                        "sysop|interface-admin|patroller|translator|wiki-creator|global-renamer|trustandsafety",
                    ),
                    ("auprop", "groups"),
                    ("aulimit", "max"),
                    ("format", "json"),
                    ("formatversion", "2"),
                ])
                .await
                .unwrap();
            let arr = data["query"]["allusers"].as_array().unwrap();
            let mut groups: HashMap<String, HashSet<String>> = HashMap::new();
            for user in arr {
                let name = user["name"].as_str().unwrap();
                if name == "⧼abusefilter-blocker⧽" || name == "Abuse filter" {
                    continue;
                }
                let mut group: HashSet<_> = user["groups"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .filter(|s| {
                        !IMPLICIT_GROUPS.contains(&s.as_str())
                            && !TEMPORAL_GROUPS.contains(&s.as_str())
                    })
                    .collect();
                // rewrite
                for (from, to) in REWRITE_GROUPS.iter() {
                    if group.contains(from.to_owned()) {
                        group.remove(from.to_owned());
                        group.insert(to.to_string());
                    }
                }

                groups.insert(name.to_string(), group);
            }

            let data2 = bot
                .api()
                .get_value(&[
                    ("action", "query"),
                    ("list", "globalallusers"),
                    ("agugroup", "steward|sre|global-sysop|global-rollbacker"),
                    ("aguprop", "groups"),
                    ("agulimit", "max"),
                    ("format", "json"),
                    ("formatversion", "2"),
                ])
                .await
                .unwrap();
            let arr2 = data2["query"]["globalallusers"].as_array().unwrap();
            for user in arr2 {
                let name = user["name"].as_str().unwrap();
                let group: HashSet<_> = user["groups"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .filter(|s| {
                        !IMPLICIT_GROUPS.contains(&s.as_str())
                            && !GLOBAL_TEMPORAL_GROUPS.contains(&s.as_str())
                    })
                    .collect();
                if !groups.contains_key(name) {
                    groups.insert(name.to_string(), group.clone());
                } else {
                    groups.get_mut(name).unwrap().extend(group.clone());
                }
                if group.contains("steward") {
                    groups
                        .get_mut(name)
                        .unwrap()
                        .insert("wiki-creator".to_string());
                }
            }

            let mut groups = groups.iter().collect::<Vec<(&String, &HashSet<_>)>>();
            groups.sort_by_key(|x| x.0);
            let mut groups = groups
                .iter()
                .map(|f| (f.0, f.1.iter().collect::<Vec<_>>()))
                .collect::<Vec<_>>();
            groups.iter_mut().for_each(|f| {
                f.1.sort();
            });

            let mut text = String::new();
            text.push_str("return {\n");
            for (name, group) in groups {
                text.push_str(&format!("    [\'{}\'] = {{", name));
                text.push_str(
                    &group
                        .iter()
                        .map(|x| format!("\'{}\'", x))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                text.push_str("},\n");
            }
            text.push_str("}\n");

            let page = bot.page("Module:Othergroups/data").unwrap();
            page.save(&text, &SaveOptions::summary(&summary("update othergroups")))
                .await
                .unwrap();
            // 1時間
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
        }
    });

    let bot_clone3 = Arc::clone(&bot);
    tokio::spawn(async move {
        let bot_clone3 = Arc::clone(&bot_clone3);
        loop {
            let bot = bot_clone3.clone();

            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

            let status = check_status(bot.clone()).await;
            if !status {
                println!("status is false");
                // 1時間
                tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;

                continue;
            }

            // remove marker
            println!("remove marker");

            let page = bot.page("Meta:Requests for permissions").unwrap();
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

            page.save(&text, &SaveOptions::summary(&summary("remove marker")))
                .await
                .unwrap();

            // 1時間
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
        }
    });

    let bot_clone4 = Arc::clone(&bot);
    tokio::spawn(async move {
        let bot_clone4 = Arc::clone(&bot_clone4);
        loop {
            let bot = bot_clone4.clone();

            tokio::time::sleep(tokio::time::Duration::from_secs(40)).await;

            let status = check_status(bot.clone()).await;
            if !status {
                println!("status is false");
                // 1時間
                tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
                continue;
            }

            println!("add status");

            let an = bot.page("Meta:Administrators'_noticeboard").unwrap();
            let mut an_text = an.wikitext().await.unwrap();
            let an_sections = extract_sections_with_titles(&an_text);
            for (_title, section) in an_sections {
                if !section.to_lowercase().contains("{{status") {
                    let new_section = "{{status}}\n".to_string() + &section.trim_start();
                    an_text = an_text.replace(&section, &new_section);
                }
            }
            an.save(&an_text, &SaveOptions::summary(&summary("add status")))
                .await
                .unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            let global = bot.page("Steward_requests/Global").unwrap();
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
            global
                .save(&global_text, &SaveOptions::summary(&summary("add status")))
                .await
                .unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            let reports = bot.page("Steward_requests/Wiki_reports").unwrap();
            let mut reports_text = reports.wikitext().await.unwrap();
            let reports_sections = extract_sections_with_titles(&reports_text);
            for (_title, section) in reports_sections {
                if !section.to_lowercase().contains("{{status") {
                    let new_section = "{{status}}\n".to_string() + &section.trim_start();
                    reports_text = reports_text.replace(&section, &new_section);
                }
            }
            reports
                .save(&reports_text, &SaveOptions::summary(&summary("add status")))
                .await
                .unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            let dc = bot.page("Steward_requests/Discussion_closure").unwrap();
            let mut dc_text = dc.wikitext().await.unwrap();
            let dc_sections = extract_sections_with_titles(&dc_text);
            for (_title, section) in dc_sections {
                if !section.to_lowercase().contains("{{status") {
                    let new_section = "{{status}}\n".to_string() + &section.trim_start();
                    dc_text = dc_text.replace(&section, &new_section);
                }
            }
            dc.save(&dc_text, &SaveOptions::summary(&summary("add status")))
                .await
                .unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            let misc = bot.page("Steward_requests/Miscellaneous").unwrap();
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
            misc.save(&misc_text, &SaveOptions::summary(&summary("add status")))
                .await
                .unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            // 1時間
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
        }
    });

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }

    #[allow(unreachable_code)]
    Ok(())
}
