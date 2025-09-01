#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = get_data().await?;
    println!("{}", &content);

    Ok(())
}

async fn get_data() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/ip").await?;
    let text = response.text().await?;

    Ok(text)
}
