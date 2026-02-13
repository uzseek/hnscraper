use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://news.ycombinator.com/";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    let document = Html::parse_document(&body);

    let selector = Selector::parse(".titleline > a").unwrap();

    for element in document.select(&selector) {
        let title = element.inner_html();
        println!("{}", title);
    }

    Ok(())
}

