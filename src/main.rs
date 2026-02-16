use scraper::{Html, Selector};
struct Article {
    title: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://news.ycombinator.com/";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse(".titleline > a").unwrap();
    let mut articles = Vec::new();

    for element in document.select(&selector) {
        let title = element.inner_html();

        let url = element
            .value()
            .attr("href")
            .unwrap_or("")
            .to_string();

        let article = Article {
            title,
            url,
        };

        articles.push(article);
    }

    for article in &articles {
        println!("{} - {}", article.title, article.url);
    }

    Ok(())
}

