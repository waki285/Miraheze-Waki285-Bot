use std::{fs::File, io::Write};

pub async fn temp(bot: &mwbot::Bot) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("test.txt")?;

    let mut result = bot
        .api()
        .get_value(&[
            ("action", "query"),
            ("format", "json"),
            ("list", "globalblocks"),
            ("formatversion", "2"),
            ("bglimit", "500"),
            ("bgprop", "address"),
        ])
        .await
        .unwrap();

    let ips = result["query"]["globalblocks"].as_array().unwrap();
    let ips = ips
        .iter()
        .map(|f| f["address"].as_str().unwrap())
        .collect::<Vec<&str>>();

    write!(file, "{}\n", ips.join("\n"))?;
    file.flush()?;

    loop {
        let bgstart = &result["continue"]["bgstart"];
        if !bgstart.is_string() {
            break;
        }
        let bgstart = bgstart.as_str().unwrap();
        result = bot
            .api()
            .get_value(&[
                ("action", "query"),
                ("format", "json"),
                ("list", "globalblocks"),
                ("formatversion", "2"),
                ("bgstart", bgstart),
                ("bglimit", "500"),
                ("bgprop", "address"),
            ])
            .await
            .unwrap();

        let ips = result["query"]["globalblocks"].as_array().unwrap();
        let ips = ips
            .iter()
            .map(|f| f["address"].as_str().unwrap())
            .collect::<Vec<&str>>();

        write!(file, "{}\n", ips.join("\n"))?;
        file.flush()?;

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    Ok(())
}
