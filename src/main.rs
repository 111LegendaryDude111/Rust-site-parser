    use reqwest;
    use scraper::{Html, Selector};
    use std::error::Error;
    use std::io;


    async fn fetch_html(url: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(url).await?.text().await?;
        Ok(response)
    }
    


    fn extract_links(html: &str) -> Vec<String> {

        
        let document = Html::parse_document(html);
        let selector = Selector::parse("a").unwrap();
       
        document
            .select(&selector)
            .filter_map(|element| element.value().attr("href"))
            .map(|href| href.to_string())
            .collect()
    }
    
    #[tokio::main]
    async fn main() {
        println!("Введите URL для парсинга:");
        let mut url = String::new();
        io::stdin().read_line(&mut url).expect("Ошибка ввода URL");
        let url = url.trim();


        match fetch_html(url).await {
            Ok(html) => {
                let links = extract_links(&html);
                println!("Найденные ссылки:");
                for link in links {
                    println!("{}", link);
                }
            }
            Err(e) => eprintln!("Ошибка: {}", e),
        }
    }
