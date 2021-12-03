macro_rules! clear {
    () => {
        "\x1b[2J\x1b[1;1H"
    };
}

macro_rules! red {
    () => {
        "\x1b[31;1m"
    };
    ($e:expr) => {
        concat!("\x1b[31;1m", $e)
    };
}

macro_rules! green {
    () => {
        "\x1b[32;1m"
    };
    ($e:expr) => {
        concat!("\x1b[32;1m", $e)
    };
}

macro_rules! yellow {
    () => {
        "\x1b[33;1m"
    };
    ($e:expr) => {
        concat!("\x1b[33;1m", $e)
    };
}

macro_rules! blue {
    () => {
        "\x1b[34;1m"
    };
    ($e:expr) => {
        concat!("\x1b[34;1m", $e)
    };
}

macro_rules! magenta {
    () => {
        "\x1b[35;1m"
    };
    ($e:expr) => {
        concat!("\x1b[35;1m", $e)
    };
}

macro_rules! cyan {
    () => {
        "\x1b[36;1m"
    };
    ($e:expr) => {
        concat!("\x1b[36;1m", $e)
    };
}

macro_rules! quit {
    () => {
        concat!(blue!("["), cyan!("q"), blue!("] "), red!("quit\n"))
    };
}

macro_rules! print_start {
    () => {
        println!(concat!(
            blue!("["),
            cyan!("h"),
            blue!("] "),
            yellow!("history\n"),
            blue!("["),
            cyan!("s"),
            blue!("] "),
            magenta!("search\n"),
            quit!()
        ));
    };
}

macro_rules! print_title {
    ($i:expr, $title:expr) => {
        println!(
            "{}{}{}{}{}{}",
            blue!("["),
            cyan!(),
            $i,
            blue!("] "),
            yellow!(),
            $title
        );
    };
    ($i:expr, $title:expr, $episode:expr, $episodes:expr) => {
        println!(
            "{}{}{}{}{}{}{}{}{}{}",
            blue!("["),
            cyan!(),
            $i,
            blue!("] "),
            yellow!(),
            $title,
            cyan!(" "),
            $episode,
            "/",
            $episodes
        );
    };
}

macro_rules! print_episodes {
    ($title:expr, $max_episode:expr) => {
        println!(
            "{}{}{}{}{}{}",
            blue!("Choose episode for "),
            yellow!(),
            $title,
            cyan!(" [1-"),
            $max_episode,
            "]\n"
        );
    };
}

macro_rules! print_language {
    ($i:expr, $language:expr) => {
        println!(
            "{}{}{}{}{}{}",
            blue!("["),
            cyan!(),
            $i,
            blue!("] "),
            yellow!(),
            $language
        );
    };
}

macro_rules! print_video {
    ($title:expr, $episode:expr, $max_episode:expr) => {
        println!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            green!("Playing "),
            yellow!(),
            $title,
            cyan!(" "),
            $episode,
            "/",
            $max_episode,
            blue!("\n\n["),
            cyan!("p"),
            blue!("] "),
            yellow!("previous\n"),
            blue!("["),
            cyan!("r"),
            blue!("] "),
            magenta!("replay\n"),
            blue!("["),
            cyan!("n"),
            blue!("] "),
            yellow!("next\n"),
            blue!("["),
            cyan!("s"),
            blue!("] "),
            magenta!("select episode\n"),
            blue!("["),
            cyan!("f"),
            blue!("] "),
            yellow!("finished\n"),
            blue!("["),
            cyan!("l"),
            blue!("] "),
            magenta!("language\n"),
            quit!()
        );
    };
}

macro_rules! read_line {
    () => {{
        use std::io::{self, Write};

        print!(concat!(blue!(": "), green!()));
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        buffer.trim().to_string()
    }};
}
