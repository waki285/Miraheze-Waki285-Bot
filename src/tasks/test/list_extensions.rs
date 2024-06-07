use std::sync::Arc;

use mwbot::SaveOptions;
use php_parser_rs::parser;

use crate::util::{check_status, summary};

#[derive(Debug)]
struct Extension {
    pub key: String,
    pub name: String,
    pub link: String,
    pub restricted: bool,
}

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
                    parser::ast::ArrayItem::KeyValue { key, double_arrow, value } => {
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
                                        parser::ast::ArrayItem::KeyValue { key, double_arrow, value } => {
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
                                        parser::ast::ArrayItem::KeyValue { key, double_arrow, value } => {
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
                                        parser::ast::ArrayItem::KeyValue { key, double_arrow, value } => {
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
                                        parser::ast::ArrayItem::KeyValue { key, double_arrow, value } => {
                                            match key {
                                                parser::ast::Expression::Literal(s) => {
                                                    match s {
                                                        parser::ast::literals::Literal::String(s) => {
                                                            if s.value.to_string() == "requires" {
                                                                match value {
                                                                    parser::ast::Expression::ShortArray(array) => {
                                                                        Some(array.items.iter().any(|item| {
                                                                            match item {
                                                                                parser::ast::ArrayItem::KeyValue { key, double_arrow, value } => {
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

    dbg!(extensions);


    Ok(())
}
