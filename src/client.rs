use serde_json::Deserializer;
use serde_json::de::IoRead;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};
use crate::{RucheResult, RucheError};
use crate::request::Request;
use crate::response::{GetResponse, SetResponse, RemoveResponse};
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
        serde_json::to_writer(&mut self.writer, &Request::Get { key });
        self.writer.flush()?;
        let resp = GetResponse::deserialize(&mut self.reader)?;
        match resp {
            GetResponse::Ok(value) => Ok(value),
            GetResponse::Err(e) => Err(RucheError::StringError(e))
        }
    }

    /// Set the value in the storage.
    pub fn set(&mut self, key: String, value: String) -> RucheResult<()> {
        serde_json::to_writer(&mut self.writer, &Request::Set { key, value });
        self.writer.flush()?;
        let resp = SetResponse::deserialize(&mut self.reader)?;
        match resp {
            SetResponse::Ok(value) => Ok(value),
            SetResponse::Err(e) => Err(RucheError::StringError(e))
        }
    }

    /// Remove the value in the storage.
    pub fn remove(&mut self, key: String) -> RucheResult<()> {
        serde_json::to_writer(&mut self.writer, &Request::Remove { key });
        self.writer.flush()?;
        let resp = RemoveResponse::deserialize(&mut self.reader)?;
        match resp {
            RemoveResponse::Ok(value) => Ok(value),
            RemoveResponse::Err(e) => Err(RucheError::StringError(e))
        }
    }
}