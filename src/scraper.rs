use std::collections::HashMap;

use serde_json::Value;

use crate::Anime;

pub fn search(query: &str) -> Vec<Anime> {
    let data = ureq::get(&format!("https://animepahe.com/api?m=search&q={}", query))
        .call()
        .unwrap()
        .into_json::<Value>()
        .unwrap();

    if data["total"].as_u64().unwrap() == 0 {
        Vec::new()
    } else {
        data["data"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| {
                let id = value["id"].as_u64().unwrap().to_string();
                let airing = value["status"].as_str().unwrap() == "Currently Airing";

                let episodes = if airing {
                    ureq::get(&format!("https://animepahe.com/api?m=release&id={}", id))
                        .call()
                        .unwrap()
                        .into_json::<Value>()
                        .unwrap()["total"]
                        .as_u64()
                        .unwrap() as usize
                } else {
                    value["episodes"].as_u64().unwrap() as usize
                };

                Anime::new(
                    id,
                    value["title"].as_str().unwrap().to_string(),
                    0,
                    episodes,
                    airing,
                    "jpn".to_string(),
                )
            })
            .collect()
    }
}

pub fn update(anime: &mut Anime) {
    let data = ureq::get(&format!(
        "https://animepahe.com/api?m=search&q={}",
        anime.title()
    ))
    .call()
    .unwrap()
    .into_json::<Value>()
    .unwrap();

    let value = data["data"]
        .as_array()
        .unwrap()
        .iter()
        .find(|value| value["id"].as_u64().unwrap().to_string() == anime.id())
        .unwrap();

    let airing = value["status"].as_str().unwrap() == "Currently Airing";

    let episodes = if airing {
        ureq::get(&format!(
            "https://animepahe.com/api?m=release&id={}",
            anime.id()
        ))
        .call()
        .unwrap()
        .into_json::<Value>()
        .unwrap()["total"]
            .as_u64()
            .unwrap() as usize
    } else {
        value["episodes"].as_u64().unwrap() as usize
    };

    anime.airing = airing;
    anime.episodes = episodes;
}

pub fn videos(anime: &Anime) -> HashMap<String, String> {
    let data: Value = ureq::get(&format!(
        "https://animepahe.com/api?m=release&id={}",
        anime.id()
    ))
    .call()
    .unwrap()
    .into_json()
    .unwrap();

    let last_page = data["last_page"].as_u64().unwrap();

    let mut episode = 0;
    for page in 1..last_page + 1 {
        let data: Value = ureq::get(&format!(
            "https://animepahe.com/api?m=release&id={}&page={}&sort=episode_asc",
            anime.id(),
            page
        ))
        .call()
        .unwrap()
        .into_json()
        .unwrap();

        for entry in data["data"].as_array().unwrap().iter() {
            episode += 1;
            if episode == anime.episode() {
                let data: Value = ureq::get(&format!(
                    "https://animepahe.com/api?m=link&p=kwik&id={}&session={}",
                    anime.id(),
                    entry["session"].as_str().unwrap()
                ))
                .call()
                .unwrap()
                .into_json()
                .unwrap();

                let mut resolutions = HashMap::new();
                let mut videos = HashMap::new();

                for (resolution, value) in data["data"].as_array().unwrap().iter().map(|value| {
                    value
                        .as_object()
                        .unwrap()
                        .iter()
                        .next()
                        .map(|(resolution, value)| (resolution.parse::<u16>().unwrap(), value))
                        .unwrap()
                }) {
                    let language = value["audio"].as_str().unwrap().to_string();

                    if let Some(resolution_old) = resolutions.get(&language) {
                        if resolution < *resolution_old {
                            continue;
                        }
                    }

                    videos.insert(
                        language.clone(),
                        value["kwik"].as_str().unwrap().to_string(),
                    );
                    resolutions.insert(language, resolution);
                }

                return videos;
            }
        }
    }

    HashMap::new()
}

pub fn url(url: &str) -> String {
    let html = ureq::get(url)
        .set("referer", "https://kwik.cx")
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    let rg = regex::Regex::new(r"Plyr\|querySelector\|document\|([^\\']+)").unwrap();

    let split: Vec<_> = rg
        .captures(&html)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split('|')
        .collect();

    format!(
        "{}://{}-{}.{}.{}.{}/{}/{}/{}/{}.{}",
        split[10],
        split[9],
        split[8],
        split[7],
        split[6],
        split[5],
        split[4],
        split[3],
        split[2],
        split[1],
        split[0]
    )
}
