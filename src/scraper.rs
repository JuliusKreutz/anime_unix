use serde_json::Value;

use crate::Anime;

pub fn search(keyword: &str) -> Vec<Anime> {
    ureq::get(&format!(
        "https://gogoanime.herokuapp.com/search?keyw={}",
        keyword
    ))
    .call()
    .unwrap()
    .into_json::<Value>()
    .unwrap()
    .as_array()
    .unwrap()
    .iter()
    .map(|value| {
        Anime::new(
            value["animeId"].as_str().unwrap().to_string(),
            value["animeTitle"].as_str().unwrap().to_string(),
            0,
            0,
            false,
        )
    })
    .collect()
}

pub fn update(anime: &mut Anime) {
    let data = ureq::get(&format!(
        "https://gogoanime.herokuapp.com/anime-details/{}",
        anime.id
    ))
    .call()
    .unwrap()
    .into_json::<Value>()
    .unwrap();

    anime.episodes = data["totalEpisodes"].as_str().unwrap().parse().unwrap();
    anime.airing = data["status"] == "Ongoing";
}

pub fn video(anime: &Anime) -> String {
    let data = ureq::get(&format!(
        "https://gogoanime.herokuapp.com/vidcdn/watch/{}-episode-{}",
        anime.id, anime.episode
    ))
    .call()
    .unwrap()
    .into_json::<Value>()
    .unwrap();

    let mut max_resolution = 0;
    let mut video = "";
    for source in data["sources"].as_array().unwrap() {
        let resolution = source["label"]
            .as_str()
            .unwrap()
            .strip_suffix(" P")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);

        if resolution > max_resolution {
            video = source["file"].as_str().unwrap();
            max_resolution = resolution;
        }
    }

    video.to_string()
}
