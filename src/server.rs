use crate::{RucheStore, RucheResult};
use std::net::{TcpStream, ToSocketAddrs, TcpListener, SocketAddr};
use std::io::{BufReader, BufWriter, Write};
use crate::request::Request;
use serde_json::Deserializer;
use crate::response::{GetResponse, SetResponse, RemoveResponse};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::borrow::BorrowMut;
use std::thread;

/// The server of a key value store.
#[derive(Clone)]
pub struct RucheServer {
    store: RucheStore
}

impl RucheServer {
    /// Create a server instance.
    pub fn new() -> Self {
        RucheServer {
            store: RucheStore::new()
        }
    }

    /// Run the server listening on the given address
    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> RucheResult<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            let mut server_clone = self.clone();
            thread::spawn(move || {
                match stream {
                    Ok(stream) => {
                        serve(&mut server_clone, stream);
                    },
                    Err(e) => error!("Connection failed: {}", e)
                }
            });
        }
        Ok(())
    }
}

fn serve(server: &mut RucheServer, stream: TcpStream) -> RucheResult<()> {
    let socket_addr = stream.peer_addr()?;
    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let req_stream = Deserializer::from_reader(reader).into_iter::<Request>();

    for req in req_stream {
        let req = req?;
        debug!("Receive request from {}: {:?}", socket_addr, req);
        match req {
            Request::Get { key } => {
                let res = match server.store.get(key) {
                    Ok(value) => GetResponse::Ok(value),
                    Err(e) => GetResponse::Err(format!("{}", e))
                };

                send_resp(&mut writer, socket_addr, &res)?
            },
            Request::Set { key, value } => {
                let res = match server.store.set(key, value) {
                    Ok(_) => SetResponse::Ok(()),
                    Err(e) => SetResponse::Err(format!("{}", e))
                };

                send_resp(&mut writer, socket_addr, &res)?
            },
            Request::Remove { key } => {
                let res = match server.store.remove(key) {
                    Ok(_) => RemoveResponse::Ok(()),
                    Err(e) => RemoveResponse::Err(format!("{}", e))
                };

                send_resp(&mut writer, socket_addr, &res)?
            }
        }
    }

    Ok(())
}

fn send_resp<'a, T: Serialize + Deserialize<'a> + Debug>(writer: &mut BufWriter<&TcpStream>,
                                                         addr: SocketAddr,
                                                         msg: &T) -> RucheResult<()> {
    serde_json::to_writer(writer.borrow_mut(), msg)?;
    writer.flush()?;
    debug!("Response send to {}: {:?}", addr, msg);

    Ok(())
}