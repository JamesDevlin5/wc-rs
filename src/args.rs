use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    pub bytes: bool,
    pub lines: bool,
    pub chars: bool,
    pub words: bool,
}

impl Config {
    pub fn new() -> Self {
        let m = App::new("Word Count")
            .about("Counts the number of lines/words/bytes in a file")
            .arg(
                Arg::with_name("bytes").short("c").long("bytes").help(
                    "The number of bytes in each input file is written to the standard output.",
                ),
            )
            .arg(
                Arg::with_name("lines").short("l").long("lines").help(
                    "The number of lines in each input file is written to the standard output.",
                ),
            )
            .arg(Arg::with_name("chars").short("m").long("chars").help(
                "The number of characters in each input file is written to the standard output.",
            ))
            .arg(
                Arg::with_name("words").short("w").long("words").help(
                    "The number of words in each input file is written to the standard output.",
                ),
            )
            .get_matches();
        Config {
            bytes: m.is_present("bytes"),
            lines: m.is_present("lines"),
            chars: m.is_present("chars"),
            words: m.is_present("words"),
        }
    }
}
