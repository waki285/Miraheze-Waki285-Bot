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
        mwbot::Bot::from_path(Path::new("./.config/mwbot.horridhenry.toml"))
            .await
            .unwrap(),
    );

    tasks_onetime::bt5::bt5(&bot).await?;

    Ok(())
}