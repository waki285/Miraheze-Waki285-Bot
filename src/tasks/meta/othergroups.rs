use std::{collections::{HashMap, HashSet}, sync::Arc};

use mwbot::SaveOptions;

use crate::{constants::{GLOBAL_TEMPORAL_GROUPS, IMPLICIT_GROUPS, REWRITE_GROUPS, TEMPORAL_GROUPS}, util::{check_status, summary}};

pub async fn othergroups(bot: &Arc<mwbot::Bot>) -> Result<(), anyhow::Error> {
    let status = check_status(bot.clone()).await;
    if !status {
        log::error!("status is false");
        return Err(anyhow::anyhow!("status is false"));
    }
    log::debug!("othergroups");
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
                .await;
    if let Err(e) = data {
        log::error!("Error getting data: {:?}", e);
        return Err(e.into());
    }
    let data = data.unwrap();
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
                !IMPLICIT_GROUPS.contains(&s.as_str()) && !TEMPORAL_GROUPS.contains(&s.as_str())
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
            ("agugroup", "steward|techteam|global-admin|global-rollbacker|wiki-mechanic"),
            ("aguprop", "groups"),
            ("agulimit", "max"),
            ("format", "json"),
            ("formatversion", "2"),
        ])
        .await;
    if let Err(e) = data2 {
        log::error!("Error getting data: {:?}", e);
        return Err(e.into());
    }
    let data2 = data2.unwrap();
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

    let page = bot.page("Module:Othergroups/data");

    if let Ok(p) = page {
        let result = p
            .save(&text, &SaveOptions::summary(&&summary("update othergroups")))
            .await;

        if let Err(e) = result {
            log::error!("Error saving page: {:?}", e);
            return Err(e.into());
        }
    } else {
        log::error!("Error retrieving page");
        return Err(anyhow::anyhow!("Error retrieving page"));
    }
    Ok(())
}
