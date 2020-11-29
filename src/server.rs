use crate::{RucheStore, RucheResult};
use crate::common::{Request, Response};
use tokio::prelude::*;
use tokio::net::{ToSocketAddrs, TcpListener, TcpStream};

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
    pub async fn run<A: ToSocketAddrs>(self, addr: A) -> RucheResult<()> {
        let listener = TcpListener::bind(addr).await?;
        loop {
            debug!("waiting for connection");
            let (mut stream, _) = listener.accept().await?;
            let mut server_clone = self.clone();
            tokio::spawn(async move {
                serve(&mut server_clone, &mut stream).await.unwrap();
            });
        }
    }
}

async fn serve(server: &mut RucheServer, stream: &mut TcpStream) -> RucheResult<()> {
    let remote_ip = stream.peer_addr().unwrap().ip().to_string();
    let remote_port = stream.peer_addr().unwrap().port().to_string();
    debug!("connection from remote: {}:{}", remote_ip, remote_port);
    loop {
        debug!("waiting for data");
        stream.readable().await?;
        let mut msg = vec![0; 1024];
        match stream.read(&mut msg).await {
            Ok(0) => {
                // Return value of `Ok(0)` signifies that the remote has closed.
                debug!("connection closed: {}:{}", remote_ip, remote_port);
                break;
            },
            Ok(n) => {
                let s: &str = std::str::from_utf8(&msg[..n]).unwrap();
                let req: Request = serde_json::from_str(s).unwrap();
                debug!("Got Request: {:?}", req);

                let response = match req {
                    Request::Get { key } => {
                        match server.store.get(key) {
                            Ok(value) => Response::Get(value),
                            Err(e) => Response::Err(format!("{}", e))
                        }
                    },
                    Request::Set { key, value } => {
                        match server.store.set(key, value) {
                            Ok(_) => Response::Set,
                            Err(e) => Response::Err(format!("{}", e))
                        }
                    },
                    Request::Remove { key } => {
                        match server.store.remove(key) {
                            Ok(_) => Response::Remove,
                            Err(e) => Response::Err(format!("{}", e))
                        }
                    }
                };

                let res_str = serde_json::to_string(&response).unwrap();
                stream.write(res_str.as_bytes()).await?;
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // According to tokio document.
                // This may still fail with `WouldBlock` if the readiness event is a false positive.
                // We just skip this situation.
                error!("{}", e.to_string());
            },
            Err(e) => {
                error!("{}", e.to_string());
            }
        }
    }

    Ok(())
}