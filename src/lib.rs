pub mod search_providers;
pub mod torrent;

use clap::ValueEnum;
use futures_util::future::join_all;
use search_providers::l337x_search::L337xSearch;
use search_providers::pirate_bay_search::PirateBaySearch;
use search_providers::torrent_csv_search::TorrentCsvSearch;
use search_providers::yts_search::YtsSearch;
use search_providers::{search_providers_from_ids, SearchProvider, SearchProviderId};
use torrent::Torrent;

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum SortMethods {
    Seeders,
    Leechers,
}

pub async fn search(
    keyword: &str,
    providers: Vec<&str>,
) -> Result<Vec<Torrent>, Box<dyn std::error::Error>> {
    let mut spis = Vec::new();
    for p in providers {
        if let Ok(spi) = SearchProviderId::from_str(&p, true) {
            spis.push(spi);
        }
    }
    let providers: Vec<Box<dyn SearchProvider>> = if spis.is_empty() {
        vec![
            Box::new(PirateBaySearch::new()),
            Box::new(L337xSearch::new()),
            Box::new(YtsSearch::new()),
            Box::new(TorrentCsvSearch::new()),
        ]
    } else {
        search_providers_from_ids(&spis)
    };

    // search for torrents
    let results = providers.iter().map(|provider| provider.search(keyword));
    let results = join_all(results).await;

    // collect torrents into one vec
    let mut torrents = vec![];
    for (result, provider) in results.into_iter().zip(providers) {
        match result {
            Ok(t) => torrents.extend(t),
            Err(err) => log::error!("{} error: {}", provider.get_name(), err),
        }
    }

    let sort_method = SortMethods::Seeders;
    match sort_method {
        SortMethods::Seeders => torrents.sort_by(Torrent::compare_seeders),
        SortMethods::Leechers => torrents.sort_by(Torrent::compare_leechers),
    };

    Ok(torrents)
}
