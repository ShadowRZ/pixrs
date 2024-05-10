use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Some(token) = std::env::args().nth(1) else {
        anyhow::bail!("Usage: (program) session-cookie illust-id")
    };
    let Some(illust_id) = std::env::args().nth(2) else {
        anyhow::bail!("Usage: (program) session-cookie illust-id")
    };
    let illust_id = <i32 as FromStr>::from_str(&illust_id)?;
    let client = pixrs::PixivClient::new(&token).await?;
    let info = client.illust_info(illust_id).await?;
    println!("Illust Info: {:#?}", info);
    Ok(())
}
