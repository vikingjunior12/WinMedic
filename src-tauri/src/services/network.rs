use std::time::Duration;

pub async fn check_internet() -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(8))
        .build();

    match client {
        Ok(c) => c
            .head("https://www.microsoft.com")
            .send()
            .await
            .is_ok(),
        Err(_) => false,
    }
}
