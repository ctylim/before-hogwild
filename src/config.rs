use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    pub n: usize,
    pub c: usize,
    pub k: usize,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config::default();
        let matches = App::new("before-hogwild")
            .author("ctylim")
            .arg(
                Arg::with_name("n")
                    .short("n")
                    .help("Sets n")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("c")
                    .short("c")
                    .value_name("Sets c")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("k")
                    .short("k")
                    .value_name("Sets k")
                    .takes_value(true),
            )
            .get_matches();
        if let Some(n) = matches.value_of("n") {
            config.n = n.parse().unwrap();
        }
        if let Some(c) = matches.value_of("c") {
            config.c = c.parse().unwrap();
        }
        if let Some(k) = matches.value_of("k") {
            config.k = k.parse().unwrap();
        }
        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            n: 10000000,
            c: 100,
            k: 1000000,
        }
    }
}
