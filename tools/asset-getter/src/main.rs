use anyhow::Result;

use asset_getter::Crawler;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut crawler = Crawler::new().await?;
    if let Err(err) = crawler.run().await {
        log::error!("{err}");
    }
    crawler.quit().await
}
