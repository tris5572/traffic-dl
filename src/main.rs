use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let content = get_data().await?;
    println!("{}", &content);

    Ok(())
}

async fn get_data() -> Result<String> {
    let response = reqwest::get("https://httpbin.org/ip").await?;
    let text = response.text().await?;

    Ok(text)
}
