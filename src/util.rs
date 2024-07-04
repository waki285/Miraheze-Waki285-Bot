#![allow(dead_code)]
use std::{collections::VecDeque, sync::Arc};

use regex::Regex;

use crate::constants::STATUS_PAGE;

pub fn summary(s: &str) -> String {
    format!("Bot: {}", s)
}

pub async fn check_status(bot: Arc<mwbot::Bot>) -> bool {
    let page = bot.page(STATUS_PAGE);
    if let Ok(p) = page {
        let text = p.wikitext().await.unwrap_or("false".to_string());
        text.contains("true")
    } else {
        false
    }
}

pub fn extract_sections_with_titles(text: &str) -> Vec<(String, String)> {
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