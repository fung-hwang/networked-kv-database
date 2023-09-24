use chrono::Local;
use clap::{Args, Parser, Subcommand};
use env_logger::Env;
use kvs::KvsClient;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(name = "kvs", author, version, about, long_about = None)]
struct Options {
    #[command(subcommand)]
    command: Commands,
    // Put "addr" variable here would be better, but we have to follow the specified command line argument format
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// set <KEY> <VALUE> [--addr IP-PORT] -> Set the value of a string key to a string
    Set(Set),
    /// get <KEY> [--addr IP-PORT] -> Get the string value of a given string key
    Get(Get),
    /// rm set <KEY> [--addr IP-PORT] -> Remove a given key
    Rm(Remove),
}

#[derive(Args, Debug)]
struct Set {
    key: String,
    value: String,
    #[arg(
        short,
        long,
        default_value = "127.0.0.1:4000",
        help = "Sets the listening address(IP:PORT)"
    )]
    addr: String,
}

#[derive(Args, Debug)]
struct Get {
    key: String,
    #[arg(
        short,
        long,
        default_value = "127.0.0.1:4000",
        help = "Sets the listening address(IP:PORT)"
    )]
    addr: String,
}

#[derive(Args, Debug)]
struct Remove {
    key: String,
    #[arg(
        short,
        long,
        default_value = "127.0.0.1:4000",
        help = "Sets the listening address(IP:PORT)"
    )]
    addr: String,
}

fn main() -> anyhow::Result<()> {
    // log init
    env_logger::Builder::from_env(Env::default().default_filter_or("trace"))
        .format(|buf, record| {
            let style = buf.default_level_style(record.level());
            writeln!(
                buf,
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                style.value(record.level()),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();

    let options = Options::parse();
    // info!("{:?}", options);

    match options.command {
        Commands::Set(Set { key, value, addr }) => {
            // debug!("Set {} {}", key, value);
            let client = KvsClient::new(addr);
            client.set(&key, &value)?;
            // debug!("Result: {:?}", rst);
        }
        Commands::Get(Get { key, addr }) => {
            // debug!("Get {}", key);
            let client = KvsClient::new(addr);
            let rst = client.get(&key)?;
            match rst {
                Some(value) => println!("{}", value),
                None => println!("Key not found"),
            }
        }
        Commands::Rm(Remove { key, addr }) => {
            // debug!("Get {}", key);
            let client = KvsClient::new(addr);
            client.remove(&key)?;
            // debug!("Result: {:?}", rst);
        }
    }

    Ok(())
}
