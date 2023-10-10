use super::*;
use crate::{Error, Result};
use log::debug;
use serde::Deserialize;
use serde_json::Deserializer;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub struct KvsClient<A: ToSocketAddrs + Clone> {
    addr: A,
}

impl<A: ToSocketAddrs + Clone> KvsClient<A> {
    /// New a KvsClient with a given address
    pub fn new(addr: A) -> Self {
        Self { addr }
    }

    fn connect(&self) -> Result<(BufReader<TcpStream>, BufWriter<TcpStream>)> {
        let stream = TcpStream::connect(self.addr.clone())?;
        let buf_reader = BufReader::new(stream.try_clone()?);
        let buf_writer = BufWriter::new(stream);

        Ok((buf_reader, buf_writer))
    }

    ///  Set the value of a string key in the server
    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        let request = Requests::Set {
            key: key.to_owned(),
            value: value.to_owned(),
        };

        // Short connection
        let (buf_reader, mut buf_writer) = self.connect()?;
        serde_json::to_writer(&mut buf_writer, &request)?;
        buf_writer.flush()?;
        debug!("Request: {:?}", request);

        let response = Response4Set::deserialize(&mut Deserializer::from_reader(buf_reader))?;
        debug!("Response: {:?}", response);
        match response {
            Response4Set::Ok(_) => Ok(()),
            Response4Set::Err(msg) => Err(Error::ErrorMessage(msg)),
        }
    }

    /// Get the value of a given key from the server
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        let request = Requests::Get {
            key: key.to_owned(),
        };

        // Short connection
        let (buf_reader, mut buf_writer) = self.connect()?;
        serde_json::to_writer(&mut buf_writer, &request)?;
        buf_writer.flush()?;
        debug!("Request: {:?}", request);

        let response: Response4Get = serde_json::from_reader(buf_reader)?;
        debug!("Response: {:?}", response);
        match response {
            Response4Get::Ok(a) => Ok(a),
            Response4Get::Err(msg) => Err(Error::ErrorMessage(msg)),
        }
    }

    /// Remove a string key in the server.
    pub fn remove(&self, key: &str) -> Result<()> {
        let request = Requests::Remove {
            key: key.to_owned(),
        };

        // Short connection
        let (buf_reader, mut buf_writer) = self.connect()?;
        serde_json::to_writer(&mut buf_writer, &request)?;
        buf_writer.flush()?;
        debug!("Request: {:?}", request);

        let response: Response4Remove = serde_json::from_reader(buf_reader)?;
        debug!("Response: {:?}", response);
        match response {
            Response4Remove::Ok(_) => Ok(()),
            Response4Remove::Err(msg) => Err(Error::ErrorMessage(msg)),
        }
    }
}
