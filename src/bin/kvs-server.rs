use chrono::Local;
use clap::{Parser, ValueEnum};
use env_logger::Env;
use kvs::*;
use log::{debug, error, info};
use std::env::current_dir;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

extern crate num_cpus;

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
enum Engine {
    Kvs, // KvStore
    Jammdb, // Jammdb
         // Redb,
         // Sled,
}

#[derive(Parser, Debug)]
#[command(name = "kvs", author, version, about, long_about = None)]
struct Options {
    #[arg(
        short,
        long,
        default_value = "127.0.0.1:4000",
        help = "Sets the listening address(IP:PORT)"
    )]
    addr: String,
    #[arg(short, long, help = "Sets the storage engine")]
    engine: Option<Engine>,
}

impl Options {
    /// Set the engine in options, according to current engine and args --engine.
    ///
    /// If --engine is specified, then ENGINE-NAME must be either "kvs", in which case the built-in engine is used,
    /// or "sled", in which case sled is used.
    ///
    /// If this is the first run (there is no data previously persisted) then the default value is "kvs".
    ///
    /// if there is previously persisted data then the default is the engine already in use.
    ///
    /// If data was previously persisted with a different engine than selected, print an error and exit with a non-zero exit code.
    ///
    // ==================================
    // cur\arg |  None |  kvs  |  sled  |
    // ----------------------------------
    //    None |  kvs  |  kvs  |  sled  |
    // ----------------------------------
    //    kvs  |  kvs  |  kvs  |  Err   |
    // ----------------------------------
    //    sled |  sled |  Err  |  sled  |
    // ==================================
    fn set_engine(&mut self) -> anyhow::Result<()> {
        let cur_engine = Self::current_engine()?;
        if cur_engine.is_none() {
            if self.engine.is_none() {
                self.engine = Some(Engine::Kvs)
            }
            // write engine type to engine file，e.g. kvs
            fs::write(
                Self::engine_file_path()?,
                format!("{:?}", self.engine.as_ref().unwrap()),
            )?;
        } else if self.engine.is_none() {
            self.engine = cur_engine;
        } else if cur_engine != self.engine {
            error!(
                "cur_engine: {:?} != options.engine: {:?}",
                cur_engine, self.engine
            );
            std::process::exit(1);
        }

        anyhow::Ok(())
    }

    /// Get current engine from engine file
    ///
    /// If there is no engine exists, return Ok(None).
    fn current_engine() -> anyhow::Result<Option<Engine>> {
        let engine_file = Self::engine_file_path()?;
        if !engine_file.exists() {
            anyhow::Ok(None)
        } else {
            let str_from_engine_file = fs::read_to_string(engine_file)?;
            match Engine::from_str(&str_from_engine_file, true) {
                Ok(engine) => {
                    debug!("Current engine: {:?}", engine);
                    anyhow::Ok(Some(engine))
                }
                Err(_err) => {
                    // err format: "invalid variant: abc"
                    error!("Unexpected engine: {:?}", str_from_engine_file);
                    anyhow::Ok(None)
                }
            }
        }
    }

    /// Get path of the engine file
    fn engine_file_path() -> anyhow::Result<PathBuf> {
        Ok(current_dir()?.join("engine"))
    }
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

    if let Err(err) = run(options) {
        error!("{:?}", err);
        std::process::exit(1);
    }

    Ok(())
}

fn run(mut options: Options) -> anyhow::Result<()> {
    options.set_engine()?;
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("Storage engine: {:?}", options.engine.as_ref().unwrap());
    info!("Listening on {}", options.addr);

    match options.engine.unwrap() {
        Engine::Kvs => {
            let engine = KvStore::open(current_dir()?.join("storage"))?;
            let threadpool = SharedQueueThreadPool::new(num_cpus::get())?;
            kvs::KvsServer::new(engine, threadpool)?.start(options.addr)?
        }
        Engine::Jammdb => {
            let engine = Jammdb::open(current_dir()?.join("storage"))?;
            let threadpool = SharedQueueThreadPool::new(num_cpus::get())?;
            kvs::KvsServer::new(engine, threadpool)?.start(options.addr)?
        } // Engine::Redb =>
    }

    Ok(())
}
