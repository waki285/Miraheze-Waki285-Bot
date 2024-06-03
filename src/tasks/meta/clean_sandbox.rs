use std::sync::Arc;

use mwbot::SaveOptions;

use crate::util::{check_status, summary};

pub async fn clean_sandbox(bot: &Arc<mwbot::Bot>) -> Result<(), anyhow::Error> {
    let status = check_status(bot.clone()).await;
    if !status {
        log::warn!("status is false");
        return Err(anyhow::anyhow!("status is false"));
    }

    log::debug!("cleaning sandbox");

    match bot.page("Meta:Sandbox") {
        Ok(page) => {
            let result = page
                .save(
                    "{{Please leave this line alone (sandbox heading)}}",
                    &SaveOptions::summary(&summary("cleaning sandbox")),
                )
                .await;

            if let Err(e) = result {
                log::error!("Error saving page: {:?}", e);
            }
        }
        Err(e) => {
            log::error!("Error retrieving page: {:?}", e);
        }
    }

    Ok(())
}
