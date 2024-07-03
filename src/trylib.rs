#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();
    println!("hi");
    let keyword = "avengers";
    let providers = vec!["pirate-bay", "torrent-csv", "yts"];
    let torrents = attractorr::search(keyword, providers).await?;
    for torrent in &torrents {
        println!("{:?}", torrent);
    }
    println!("torrents: {}", torrents.len());
    println!("bye");
    Ok(())
}
