#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://news.ycombinator.com/";

    let response = reqwest::get(url).await?;

    println!("Status: {}", response.status());

    let body = response.text().await?;

    println!("Body length: {}", body.len());

    Ok(())
}

