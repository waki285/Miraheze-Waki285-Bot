mod constants;
mod tasks;
mod util;

use std::{path::Path, sync::Arc};

use tasks::{
    meta::{
        add_status::add_status, clean_sandbox::clean_sandbox, othergroups::othergroups,
        remove_marker::remove_marker, srg_mark_done::srg_mark_done, update_rc::update_rc,
        list_extensions::list_extensions,
    },
    //test::list_extensions::list_extensions,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_module(
            "main",
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

    /*
    // debug
    let bot_clone0 = Arc::clone(&bot);
    tokio::spawn(async move {
        let bot = Arc::clone(&bot_clone0);
        loop {
            list_extensions(&bot).await.ok();

            // 1時間
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
        }
    });
    */

    let bot_clone = Arc::clone(&bot);
    tokio::spawn(async move {
        let bot = Arc::clone(&bot_clone);
        loop {
            let result = clean_sandbox(&bot).await;
            match result {
                Ok(()) => {
                    // 12時間
                    tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60 * 12)).await;
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    // 1時間
                    tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
                }
            }
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    let bot_clone2 = Arc::clone(&bot);
    tokio::spawn(async move {
        let bot_clone2 = Arc::clone(&bot_clone2);
        loop {
            let bot = bot_clone2.clone();

            othergroups(&bot).await.ok();
            // 1時間
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    let bot_clone3 = Arc::clone(&bot);
    tokio::spawn(async move {
        let bot_clone3 = Arc::clone(&bot_clone3);
        loop {
            let bot = bot_clone3.clone();

            remove_marker(&bot).await.ok();
            // 1時間
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    let bot_clone4 = Arc::clone(&bot);
    tokio::spawn(async move {
        let bot_clone4 = Arc::clone(&bot_clone4);
        loop {
            let bot = bot_clone4.clone();

            add_status(&bot).await.ok();

            // 1時間
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    let bot_clone5 = Arc::clone(&bot);
    tokio::spawn(async move {
        let bot = Arc::clone(&bot_clone5);
        loop {
            update_rc(&bot).await.ok();

            // 1時間
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    let bot_clone6 = Arc::clone(&bot);
    tokio::spawn(async move {
        let bot = Arc::clone(&bot_clone6);
        loop {
            srg_mark_done(&bot).await.ok();

            // 1時間
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    let bot_clone7 = Arc::clone(&bot);
    tokio::spawn(async move {
        let bot = Arc::clone(&bot_clone7);
        loop {
            list_extensions(&bot).await.ok();

            // 12時間
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60 * 12)).await;
        }
    });

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }

    #[allow(unreachable_code)]
    Ok(())
}
