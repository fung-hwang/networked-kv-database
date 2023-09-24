use crate::{common::*, KvsEngine, Result};
use log::{debug, error};
use serde::Deserialize;
use serde_json::Deserializer;
use std::{
    env::current_dir,
    io::{BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

pub struct KvsServer<E: KvsEngine> {
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    pub fn new() -> Result<Self> {
        Ok(Self {
            engine: E::open(current_dir()?.join("database"))?,
        })
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
                    self.handle_connection(stream)?;
                }
                Err(err) => {
                    error!("TCP connection failed: {err}");
                }
            }
        }

        Ok(())
    }

    /// Receive requests, execute command and send response
    fn handle_connection(&mut self, stream: TcpStream) -> Result<()> {
        let buf_reader = BufReader::new(&stream);
        let mut buf_writer = BufWriter::new(&stream);
        // Don't use serde_json::from_reader(), this function will not return if the stream does not end
        let request = Requests::deserialize(&mut Deserializer::from_reader(buf_reader))?;
        // let requests = serde_json::Deserializer::from_reader(buf_reader).into_iter::<Requests>();

        debug!("Receive request: {:?}", request);
        match request {
            Requests::Set { key, value } => {
                let rst = self.engine.set(key, value);
                let response: Response4Set = match rst {
                    Ok(_) => Response4Set::Ok(()),
                    Err(err) => Response4Set::Err(format!("{}", err)),
                };
                serde_json::to_writer(&mut buf_writer, &response)?;
                buf_writer.flush()?;
                debug!("Send response: {:?}", response);
            }
            Requests::Get { key } => {
                let rst = self.engine.get(key);
                let response: Response4Get = match rst {
                    Ok(value) => Response4Get::Ok(value),
                    Err(err) => Response4Get::Err(format!("{}", err)),
                };
                serde_json::to_writer(&mut buf_writer, &response)?;
                buf_writer.flush()?;
                debug!("Send response: {:?}", response);
            }
            Requests::Remove { key } => {
                let rst = self.engine.remove(key);
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
