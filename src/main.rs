use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};

mod saves;
mod scraper;
#[macro_use]
mod macros;

#[derive(Clone, Serialize, Deserialize)]
pub struct Anime {
    id: String,
    title: String,
    episode: usize,
    episodes: usize,
    airing: bool,
}

impl Anime {
    pub fn new(id: String, title: String, episode: usize, episodes: usize, airing: bool) -> Self {
        Self {
            id,
            title,
            episode,
            episodes,
            airing,
        }
    }
}

fn main() {
    clear();

    loop {
        print_start!();
        let command = read_line!();
        clear();

        match command.as_str() {
            "h" => history(),
            "s" => search(),
            "q" => {
                break;
            }
            _ => {}
        }
    }
}

fn history() {
    let history = saves::get_history();

    if history.is_empty() {
        return;
    }

    let mut unfinished = Vec::new();
    let mut airing = Vec::new();
    let mut finished = Vec::new();

    for mut anime in history {
        if anime.airing {
            scraper::update(&mut anime);
        }

        if anime.episode <= anime.episodes {
            unfinished.push(anime);
        } else if anime.airing {
            airing.push(anime);
        } else {
            finished.push(anime);
        }
    }

    let selected = loop {
        let mut i = 0;

        if !unfinished.is_empty() {
            println!(concat!(blue!(), "Unfinished:"));
        }

        for anime in &unfinished {
            print_title!(i, anime.title, anime.episode, anime.episodes);
            i += 1;
        }

        if !unfinished.is_empty() {
            println!();
        }

        if !airing.is_empty() {
            println!(concat!(blue!(), "Airing:"));
        }

        for anime in &airing {
            print_title!(i, anime.title, "-", anime.episodes);
            i += 1;
        }

        if !airing.is_empty() {
            println!();
        }

        if !finished.is_empty() {
            println!(concat!(blue!(), "Finished:"));
        }

        for anime in &finished {
            print_title!(i, anime.title, "-", anime.episodes);
            i += 1;
        }

        if !finished.is_empty() {
            println!();
        }

        println!(quit!());
        let command = read_line!();
        clear();

        if command == "q" {
            return;
        }

        let selected = command.parse::<usize>();

        if selected.is_err() {
            continue;
        }

        let selected = selected.unwrap();

        if selected < i {
            break selected;
        }
    };

    let mut all = Vec::new();
    all.extend(unfinished);
    all.extend(airing);
    all.extend(finished);

    let mut anime = all.remove(selected);

    if anime.episode > anime.episodes {
        if anime.airing {
            anime.episode = anime.episodes;
        } else {
            anime.episode = 1;
        }
    }

    play(anime);
}

fn search() {
    let mut animes = loop {
        println!(concat!(blue!(), "Search\n"));
        let keyword = read_line!();
        clear();

        if keyword.is_empty() {
            return;
        }

        let animes = scraper::search(&keyword);

        if !animes.is_empty() {
            break animes;
        }
    };

    let selected = loop {
        for (i, anime) in animes.iter().enumerate() {
            print_title!(i, anime.title);
        }

        println!(quit!());
        let command = read_line!();
        clear();

        if command == "q" {
            return;
        }

        let selected = command.parse::<usize>();

        if selected.is_err() {
            continue;
        }

        let selected = selected.unwrap();

        if selected < animes.len() {
            break selected;
        }
    };

    let mut anime = animes.remove(selected);
    scraper::update(&mut anime);

    let episode = loop {
        print_episodes!(anime.title, anime.episodes);
        let selected = read_line!();
        clear();

        if selected.is_empty() {
            return;
        }

        let selected = selected.parse();

        if selected.is_err() {
            continue;
        }

        let selected = selected.unwrap();

        if selected > 0 && selected <= anime.episodes {
            break selected;
        }
    };

    anime.episode = episode;

    play(anime);
}

fn play(mut anime: Anime) {
    let mut child = Command::new("mpv")
        .args(&[
            "--http-header-fields=Referer: https://goload.pro",
            "--volume=40",
            &scraper::video(&anime),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .unwrap();
    saves::add_history(&anime);

    loop {
        print_video!(anime.title, anime.episode, anime.episodes);
        let command = read_line!();
        clear();

        match command.as_str() {
            "p" => {
                anime.episode -= 1;

                if anime.episode == 0 {
                    let _ = child.kill();

                    return;
                }
            }
            "r" => {}
            "n" => {
                anime.episode += 1;

                if anime.episode > anime.episodes {
                    let _ = child.kill();

                    saves::add_history(&anime);
                    return;
                }
            }
            "s" => {
                let exit = loop {
                    print_episodes!(anime.title, anime.episodes);
                    let selected = read_line!();
                    clear();

                    if selected.is_empty() {
                        break true;
                    }

                    let selected = selected.parse();

                    if selected.is_err() {
                        continue;
                    }

                    let selected = selected.unwrap();

                    if selected > 0 && selected <= anime.episodes {
                        anime.episode = selected;
                        break false;
                    }
                };

                if exit {
                    continue;
                }
            }
            "f" => {
                let _ = child.kill();

                anime.episode += 1;

                saves::add_history(&anime);
                return;
            }
            "q" => {
                let _ = child.kill();
                return;
            }
            _ => {
                continue;
            }
        }

        let _ = child.kill();
        child = Command::new("mpv")
            .args(&[
                "--http-header-fields=Referer: https://goload.pro",
                "--volume=40",
                &scraper::video(&anime),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .spawn()
            .unwrap();
        saves::add_history(&anime);
    }
}

fn clear() {
    print!(clear!());
}
