use clap::{App, Arg};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

const DEFAULT_LENGTH: usize = 16;

fn main() {
    let length_option = Arg::with_name("length")
    .short("l")
    .long("length")
    .takes_value(true);

    let app = App::new("Laughing Spork")
        .version("0.1")
        .author("Kristoffer Opsahl")
        .about("A presumably secure password generator, written in rust.")
        .arg(length_option);

    let length = app
        .get_matches()
        .value_of("length")
        .map(|s| s.parse::<usize>().ok())
        .flatten()
        .unwrap_or_else(|| DEFAULT_LENGTH);
    println!("{}", generate_password(length));
}

fn generate_password(length: usize) -> String {
    let mut rng = thread_rng();
    return iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(length)
        .collect();
}
