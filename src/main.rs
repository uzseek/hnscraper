use scraper::{Html, Selector};
struct Article {
    title: String,
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
        let article = Article {
            title,
        };
        articles.push(article);
    }

    for article in &articles {
        println!("{}", article.title);
    }

    Ok(())
}

