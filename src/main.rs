use args::Config;
use std::fs::File;
use wc_rs::Count;

mod args;

fn main() {
    let cfg = Config::new();
    println!("{:?}", cfg);
    let mut c = Count::new();
    let f = File::open("t").unwrap();
    c.process(f);
    println!("{:?}", c);
}
