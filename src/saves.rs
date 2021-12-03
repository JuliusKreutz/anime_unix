use std::{
    env,
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use crate::Anime;

pub fn get_history() -> Vec<Anime> {
    let history = env::var("XDG_CACHE_HOME")
        .map(|s| format!("{}/anime/history.json", s))
        .unwrap_or(format!(
            "{}/.cache/anime/history.json",
            env::var("HOME").unwrap()
        ));

    if !Path::new(&history).exists() {
        return Vec::new();
    }

    serde_json::from_reader(BufReader::new(File::open(history).unwrap())).unwrap_or_default()
}

pub fn add_history(anime: &Anime) {
    let mut history = env::var("XDG_CACHE_HOME")
        .map(|s| format!("{}/anime", s))
        .unwrap_or(format!("{}/.cache/anime", env::var("HOME").unwrap()));

    if !Path::new(&history).exists() {
        fs::create_dir_all(&history).unwrap();
    }

    history.push_str("/history.json");

    let mut animes = get_history();

    if let Some(position) = animes.iter().position(|a| a.id == anime.id) {
        animes.remove(position);
    }

    animes.push(anime.clone());

    fs::write(history, serde_json::to_string(&animes).unwrap()).unwrap();
}
