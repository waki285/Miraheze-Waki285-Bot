#![allow(unused_imports)]
#![allow(unused_variables)]
use std::{fs::File, io::Write};

use walkdir::WalkDir;

#[allow(dead_code)]
pub async fn temp(bot: &mwbot::Bot) -> Result<(), Box<dyn std::error::Error>> {
    for e in WalkDir::new("./xmls/images").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            println!("{}", e.path().display());
        }
    }

    Ok(())
}
