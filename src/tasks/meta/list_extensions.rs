use std::sync::Arc;

use mwbot::SaveOptions;
use once_cell::sync::Lazy;
use php_parser_rs::parser;
use regex::Regex;
use itertools::Itertools;

use crate::util::{check_status, summary};

#[derive(Debug)]
struct Extension {
    pub key: String,
    pub name: String,
    pub link: String,
    pub restricted: bool,
}

const PRELOADED_EXTENSIONS: [&str; 11] = [
    "categorytree",
    "cite",
    "citethispage",
    "darkmode",
    "globaluserpage",
    "mobilefrontend",
    "purge",
    "syntaxhighlight_geshi",
    "urlshortener",
    "wikieditor",
    "wikiseo"
];

const MW_LINK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^https?://(?:www\.)mediawiki\.org\/wiki\/(.+)").unwrap());
const COLUMNS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)<!-- This section is edited by bot. CHANGES MAY BE OVERRIDDEN. If you wish to make changes to the layout, please contact \[\[User:Waki285\]\]. -->\n\{\{Columns\|count=3(\s|\S)*?\}\}").unwrap());

pub async fn list_extensions(bot: &Arc<mwbot::Bot>) -> Result<(), anyhow::Error> {
    let status = check_status(bot.clone()).await;
    if !status {
        log::warn!("status is false");
        return Err(anyhow::anyhow!("status is false"));
    }

    log::debug!("list extensions");

    let extensions_raw = reqwest::get("https://raw.githubusercontent.com/miraheze/mw-config/master/ManageWikiExtensions.php")
        .await?
        .text()
        .await?;

    let extensions = parser::parse(extensions_raw.as_bytes()).unwrap();

    // get $wgManageWikiExtensions
    let extensions = extensions.iter().find_map(|stmt| {
        match stmt {
            parser::ast::Statement::Expression(stmt) => {
                match &stmt.expression {
                    parser::ast::Expression::AssignmentOperation(stmt) => {
                        match stmt.left() {
                            parser::ast::Expression::Variable(var) => {
                                match var {
                                    parser::ast::variables::Variable::SimpleVariable(var) => {
                                        if var.name.to_string() == "$wgManageWikiExtensions" {
                                            let right = stmt.right().clone();
                                            Some(right)
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None
                                }
                            }
                            _ => None
                        }
                    }
                    _ => None
                }
            }
            _ => None
        }
    }).unwrap();

    let extensions = match extensions {
        parser::ast::Expression::ShortArray(array) => {
            array.items.iter().map(|item| {
                match item {
                    parser::ast::ArrayItem::KeyValue { key, double_arrow: _, value } => {
                        match value {
                            parser::ast::Expression::ShortArray(array) => {
                                let key = match key {
                                    parser::ast::Expression::Literal(s) => {
                                        match s {
                                            parser::ast::literals::Literal::String(s) => s.value.to_string(),
                                            _ => "".to_string()
                                        }
                                    }
                                    _ => "".to_string()
                                };
                                let is_skin = array.items.iter().any(|item| {
                                    match item {
                                        parser::ast::ArrayItem::KeyValue { key, double_arrow: _, value } => {
                                            match key {
                                                parser::ast::Expression::Literal(s) => {
                                                    match s {
                                                        parser::ast::literals::Literal::String(s) => {
                                                            if s.value.to_string() == "section" {
                                                                match value {
                                                                    parser::ast::Expression::Literal(s) => {
                                                                        match s {
                                                                            parser::ast::literals::Literal::String(s) => s.value.to_string() == "skins",
                                                                            _ => false
                                                                        }
                                                                    }
                                                                    _ => false
                                                                }
                                                            } else {
                                                                false
                                                            }
                                                        }
                                                        _ => false
                                                    }
                                                }
                                                _ => false
                                            }
                                        }
                                        _ => false
                                    }
                                });
                                if is_skin {
                                    return Extension {
                                        key: "".to_string(),
                                        name: "".to_string(),
                                        link: "".to_string(),
                                        restricted: false
                                    };
                                }
                                let name = array.items.iter().find_map(|item| {
                                    match item {
                                        parser::ast::ArrayItem::KeyValue { key, double_arrow: _, value } => {
                                            match key {
                                                parser::ast::Expression::Literal(s) => {
                                                    match s {
                                                        parser::ast::literals::Literal::String(s) => {
                                                            if s.value.to_string() == "name" {
                                                                match value {
                                                                    parser::ast::Expression::Literal(s) => {
                                                                        match s {
                                                                            parser::ast::literals::Literal::String(s) => Some(s.value.to_string()),
                                                                            _ => None
                                                                        }
                                                                    }
                                                                    _ => None
                                                                }
                                                            } else {
                                                                None
                                                            }
                                                        }
                                                        _ => None
                                                    }
                                                }
                                                _ => None
                                            }
                                        }
                                        _ => None
                                    }
                                }).unwrap();
                                let link = array.items.iter().find_map(|item| {
                                    match item {
                                        parser::ast::ArrayItem::KeyValue { key, double_arrow: _, value } => {
                                            match key {
                                                parser::ast::Expression::Literal(s) => {
                                                    match s {
                                                        parser::ast::literals::Literal::String(s) => {
                                                            if s.value.to_string() == "linkPage" {
                                                                match value {
                                                                    parser::ast::Expression::Literal(s) => {
                                                                        match s {
                                                                            parser::ast::literals::Literal::String(s) => Some(s.value.to_string()),
                                                                            _ => None
                                                                        }
                                                                    }
                                                                    _ => None
                                                                }
                                                            } else {
                                                                None
                                                            }
                                                        }
                                                        _ => None
                                                    }
                                                }
                                                _ => None
                                            }
                                        }
                                        _ => None
                                    }
                                }).unwrap();
                                let restricted = array.items.iter().find_map(|item| {
                                    match item {
                                        parser::ast::ArrayItem::KeyValue { key, double_arrow: _, value } => {
                                            match key {
                                                parser::ast::Expression::Literal(s) => {
                                                    match s {
                                                        parser::ast::literals::Literal::String(s) => {
                                                            if s.value.to_string() == "requires" {
                                                                match value {
                                                                    parser::ast::Expression::ShortArray(array) => {
                                                                        Some(array.items.iter().any(|item| {
                                                                            match item {
                                                                                parser::ast::ArrayItem::KeyValue { key, double_arrow: _, value } => {
                                                                                    match key {
                                                                                        parser::ast::Expression::Literal(s) => {
                                                                                            match s {
                                                                                                parser::ast::literals::Literal::String(s) => {
                                                                                                    if s.value.to_string() == "permissions" {
                                                                                                        match value {
                                                                                                            parser::ast::Expression::ShortArray(array) => {
                                                                                                                array.items.iter().any(|item| {
                                                                                                                    match item {
                                                                                                                        parser::ast::ArrayItem::Value { value } => {
                                                                                                                            match value {
                                                                                                                                parser::ast::Expression::Literal(s) => {
                                                                                                                                    match s {
                                                                                                                                        parser::ast::literals::Literal::String(s) => {
                                                                                                                                            s.value.to_string() == "managewiki-restricted"
                                                                                                                                        }
                                                                                                                                        _ => false
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                _ => false
                                                                                                                            }
                                                                                                                        }
                                                                                                                        _ => false
                                                                                                                    }
                                                                                                                })
                                                                                                            }
                                                                                                            _ => false
                                                                                                        }
                                                                                                    } else {
                                                                                                        false
                                                                                                    }
                                                                                                },
                                                                                                _ => false
                                                                                            }
                                                                                        }
                                                                                        _ => false
                                                                                    }
                                                                                }
                                                                                _ => false
                                                                            }
                                                                        }))
                                                                    }
                                                                    _ => None
                                                                }
                                                            } else {
                                                                None
                                                            }
                                                        }
                                                        _ => None
                                                    }
                                                }
                                                _ => None
                                            }
                                        }
                                        _ => None
                                    }
                                }).unwrap();
                                Extension {
                                    key,
                                    name,
                                    link,
                                    restricted
                                }
                            }
                            _ => Extension {
                                key: "".to_string(),
                                name: "".to_string(),
                                link: "".to_string(),
                                restricted: false
                            }
                        }
                    }
                    _ => Extension {
                        key: "".to_string(),
                        name: "".to_string(),
                        link: "".to_string(),
                        restricted: false
                    }
                }
            }).filter(|f| f.name != "").collect::<Vec<Extension>>()
        }
        _ => vec![]
    };

    let extensions = extensions.iter().filter(|ext| !PRELOADED_EXTENSIONS.contains(&ext.key.as_str())).collect::<Vec<&Extension>>();
    let extensions = extensions.iter().sorted_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())).cloned().collect::<Vec<&Extension>>();

    let mut text = String::new();
    text.push_str("{{Columns|count=3|\n");
    for ext in extensions {
        text.push_str("* ");
        if let Some(caps) = MW_LINK_REGEX.captures(&ext.link) {
            text.push_str(&format!("[[mw:{}|{}]]", &caps[1], &ext.name));
        } else {
            text.push_str(&format!("[{} {}]", &ext.link, &ext.name));
        }
        if ext.restricted {
            text.push_str(" (Restricted)");
        }
        text.push_str("\n");
    }
    text.push_str("}}");
    
    let page = bot.page("Extensions")?;
    let content = page.wikitext().await?;
    let content = COLUMNS_REGEX.replace(&content, &text);
    page.save(content.to_string(), &SaveOptions::summary(&summary("List extensions (TESTING)"))).await?;

    Ok(())
}
