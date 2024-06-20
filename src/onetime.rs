mod constants;
mod tasks_onetime;
mod util;

use std::{path::Path, sync::Arc};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_module(
            "onetime",
            if cfg!(debug_assertions) {
                log::LevelFilter::Trace
            } else {
                log::LevelFilter::Debug
            },
        )
        .init();

    let bot = Arc::new(
        mwbot::Bot::from_path(Path::new("./.config/mwbot.toml"))
            .await
            .unwrap(),
    );

    tasks_onetime::temp::temp(&bot).await?;

    Ok(())
}