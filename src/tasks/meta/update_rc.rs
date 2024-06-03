use std::sync::Arc;

use mwbot::SaveOptions;

use crate::util::{check_status, summary};

pub async fn update_rc(bot: &Arc<mwbot::Bot>) -> Result<(), anyhow::Error> {
    let status = check_status(bot.clone()).await;
    if !status {
        log::warn!("status is false");
        return Err(anyhow::anyhow!("status is false"));
    }

    log::debug!("update rc/requests");

    match bot.page("Meta:RecentChanges/Requests") {
        Ok(page) => {
            let mut string = "<!-- This page is edited by bot. CHANGES MAY BE OVERRIDDEN. If you wish to make changes to the layout, please contact [[User:Waki285]]. -->* [[Meta:Requests for permissions|Requests for permissions]]".to_string();
            let rfp = bot.page("Meta:Requests for permissions");
            if let Err(e) = rfp {
                log::error!("Error retrieving page: {:?}", e);
                return Err(e.into());
            }
            let rfp = rfp.unwrap();
            let rfp_text = rfp.wikitext().await.unwrap();
            if rfp_text.contains("{{marker|rfp_v}}") {
                string.push_str(
                    format!(
                        " ('''[[Meta:Requests_for_permissions|{}]]''')",
                        rfp_text.matches("{{marker|rfp_v}}").count()
                    )
                    .as_str(),
                );
            }
            string.push_str(" • [[Requests for global permissions]]");
            let rgp = bot.page("Requests for global permissions");
            if let Err(e) = rgp {
                log::error!("Error retrieving page: {:?}", e);
                return Err(e.into());
            }
            let rgp = rgp.unwrap();
            let rgp_text = rgp.wikitext().await.unwrap();
            if rgp_text.contains("{{marker|rfgp}}") {
                string.push_str(
                    format!(
                        " ('''[[Requests_for_global_permissions|{}]]''')",
                        rgp_text.matches("{{marker|rfgp}}").count()
                    )
                    .as_str(),
                );
            }
            string.push_str(" • [[Requests for Stewardship]]");
            let rs = bot.page("Requests for Stewardship");
            if let Err(e) = rs {
                log::error!("Error retrieving page: {:?}", e);
                return Err(e.into());
            }
            let rs = rs.unwrap();
            let rs_text = rs.wikitext().await.unwrap();
            if rs_text.contains("{{marker|rfs}}") {
                string.push_str(
                    format!(
                        " ('''[[Requests_for_Stewardship|{}]]''')",
                        rs_text.matches("{{marker|rfs}}").count()
                    )
                    .as_str(),
                );
            }
            string.push_str(" • [[Steward requests]] • [[Community portal]] • [[Meta:Administrators' noticeboard|Meta Administrators' noticeboard]]  • [[Meta:Community portal|Meta Community portal]]");
            let result = page
                .save(
                    &string,
                    &SaveOptions::summary(&summary("update rc/requests")),
                )
                .await;

            if let Err(e) = result {
                println!("Error saving page: {:?}", e);
            }

            return Ok(());
        }
        Err(e) => {
            log::error!("Error retrieving page: {:?}", e);
            return Err(e.into());
        }
    }
}
