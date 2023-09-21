use core::panic;

use clap::{Args, Parser, Subcommand};
// use kvs::KvStore;

#[derive(Parser, Debug)]
#[command(name = "kvs", author, version, about, long_about = None)]
struct Options {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Set the value of a string key to a string
    Set(Set),
    /// Get the string value of a given string key
    Get(Get),
    /// Remove a given key
    Rm(Remove),
}

#[derive(Args, Debug)]
struct Set {
    key: String,
    value: String,
}

#[derive(Args, Debug)]
struct Get {
    key: String,
}

#[derive(Args, Debug)]
struct Remove {
    key: String,
}

fn main() {
    let options = Options::parse();
    // println!("{:?}", options);

    // let kvstore = KvStore::new();

    match &options.command {
        Commands::Set(_) => {
            eprintln!("set unimplemented");
            panic!()
        }
        Commands::Get(_) => {
            eprintln!("get unimplemented");
            panic!()
        }
        Commands::Rm(_) => {
            eprintln!("remove unimplemented");
            panic!()
        }
    }
}
