use serde_json::Deserializer;
use serde_json::de::IoRead;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};
use crate::{RucheResult, RucheError};
use crate::common::{Request, Response};
use serde::Deserialize;

/// Client side for ruche command line.
pub struct RucheClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>
}

impl RucheClient {
    /// Create ruche command client client.
    pub fn new<A: ToSocketAddrs>(addr: A) -> RucheResult<Self> {
        let reader = TcpStream::connect(addr)?;
        let writer = reader.try_clone()?;

        Ok(RucheClient {
            reader: Deserializer::from_reader(BufReader::new(reader)),
            writer: BufWriter::new(writer)
        })
    }

    /// Get the value in the storage.
    pub fn get(&mut self, key: String) -> RucheResult<Option<String>> {
        serde_json::to_writer(&mut self.writer, &Request::Get { key })?;
        self.writer.flush()?;
        let resp = Response::deserialize(&mut self.reader)?;
        match resp {
            Response::Get(value) => Ok(value),
            Response::Err(e) => Err(RucheError::StringError(e)),
            _ => Err(RucheError::StringError("Invalid response".to_owned()))
        }
    }

    /// Set the value in the storage.
    pub fn set(&mut self, key: String, value: String) -> RucheResult<()> {
        serde_json::to_writer(&mut self.writer, &Request::Set { key, value })?;
        self.writer.flush()?;
        let resp = Response::deserialize(&mut self.reader)?;
        match resp {
            Response::Set => Ok(()),
            Response::Err(e) => Err(RucheError::StringError(e)),
            _ => Err(RucheError::StringError("Invalid response".to_owned()))
        }
    }

    /// Remove the value in the storage.
    pub fn remove(&mut self, key: String) -> RucheResult<()> {
        serde_json::to_writer(&mut self.writer, &Request::Remove { key })?;
        self.writer.flush()?;
        let resp = Response::deserialize(&mut self.reader)?;
        match resp {
            Response::Remove => Ok(()),
            Response::Err(e) => Err(RucheError::StringError(e)),
            _ => Err(RucheError::StringError("Invalid response".to_owned()))
        }
    }
}