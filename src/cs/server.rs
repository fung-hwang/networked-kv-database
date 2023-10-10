use super::*;
use crate::{KvsEngine, Result, ThreadPool};
use log::{debug, error};
use serde::Deserialize;
use serde_json::Deserializer;
use std::{
    io::{BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

pub struct KvsServer<E: KvsEngine, T: ThreadPool> {
    engine: E,
    threadpool: T,
}

impl<E: KvsEngine, T: ThreadPool> KvsServer<E, T> {
    pub fn new(engine: E, threadpool: T) -> Result<Self> {
        Ok(Self { engine, threadpool })
    }

    /// Run the server listening on the given address
    pub fn start<A: ToSocketAddrs>(&mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr).unwrap_or_else(|err| {
            error!("Unable to bind addr: {err}");
            std::process::exit(1)
        });

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let engine = self.engine.clone();
                    self.threadpool.spawn(move || {
                        if let Err(err) = Self::handle_connection(engine, stream) {
                            error!("Handle TCP connection error: {err}");
                        }
                    })
                }
                Err(err) => {
                    error!("TCP connection failed: {err}");
                }
            }
        }

        Ok(())
    }

    /// Receive requests, execute command and send response
    fn handle_connection(engine: E, stream: TcpStream) -> Result<()> {
        let buf_reader = BufReader::new(&stream);
        let mut buf_writer = BufWriter::new(&stream);
        // Don't use serde_json::from_reader(), this function will not return if the stream does not end
        let request = Requests::deserialize(&mut Deserializer::from_reader(buf_reader))?;
        // let requests = serde_json::Deserializer::from_reader(buf_reader).into_iter::<Requests>();

        debug!("Receive request: {:?}", request);
        match request {
            Requests::Set { key, value } => {
                let rst = engine.set(key, value);
                let response: Response4Set = match rst {
                    Ok(_) => Response4Set::Ok(()),
                    Err(err) => Response4Set::Err(format!("{}", err)),
                };
                serde_json::to_writer(&mut buf_writer, &response)?;
                buf_writer.flush()?;
                debug!("Send response: {:?}", response);
            }
            Requests::Get { key } => {
                let rst = engine.get(key);
                let response: Response4Get = match rst {
                    Ok(value) => Response4Get::Ok(value),
                    Err(err) => Response4Get::Err(format!("{}", err)),
                };
                serde_json::to_writer(&mut buf_writer, &response)?;
                buf_writer.flush()?;
                debug!("Send response: {:?}", response);
            }
            Requests::Remove { key } => {
                let rst = engine.remove(key);
                let response: Response4Remove = match rst {
                    Ok(_) => Response4Remove::Ok(()),
                    Err(err) => Response4Remove::Err(format!("{}", err)),
                };
                serde_json::to_writer(&mut buf_writer, &response)?;
                buf_writer.flush()?;
                debug!("Send response: {:?}", response);
            }
        }

        Ok(())
    }
}
