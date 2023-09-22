use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use kvs::{KvStore, KvsEngine};
use std::env::current_dir;

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

fn main() -> Result<()> {
    let options = Options::parse();
    // println!("{:?}", options);

    let mut kvstore = KvStore::open(current_dir()?.join("kvstore.db"))?;

    match &options.command {
        Commands::Set(Set { key, value }) => {
            kvstore.set(key.to_owned(), value.to_owned())?;
        }
        Commands::Get(Get { key }) => {
            if let Some(value) = kvstore.get(key.to_owned())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Commands::Rm(Remove { key }) => {
            let rst = kvstore.remove(key.to_owned());
            if let Err(kvs::Error::KeyNotFound) = rst {
                // `kvs rm <KEY>` should print "Key not found" for an empty database and exit with non-zero code.
                println!("Key not found");
                std::process::exit(-1);
            } else {
                rst?;
            }
        }
    }

    Ok(())
}
