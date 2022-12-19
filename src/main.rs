#![doc = include_str!("../README.md")]

use config::Config;
use log::{debug, info};
use message::data_server::*;
use std::error::Error;
use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

mod config;
mod message;

/// Handles connection from cache server
async fn connection_handler(
    mut socket: TcpStream,
    db: Arc<sled::Db>,
) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);

    loop {
        // Read request and deserialize
        let mut raw_request = Vec::new();
        reader.read_until(0u8, &mut raw_request).await?;
        let request = postcard::from_bytes_cobs::<Request>(&mut raw_request)?;
        info!(
            "From {} got {:?}",
            writer.peer_addr()?,
            request
        );

        // Handle request
        let mut response = match request {
            Request::Load(request) => match db.get(request.key)? {
                Some(hash) => Response::Load(response::Load {
                    hash: Some(hash.to_vec()),
                }),
                None => Response::Load(response::Load { hash: None }),
            },
            Request::Store(request) => match db.insert(request.key, request.hash) {
                Ok(_) => Response::Store(response::Store { success: true }),
                Err(_) => Response::Store(response::Store { success: false }),
            },
        };

        // Write response
        let mut raw_response = postcard::to_stdvec_cobs(&mut response)?;
        writer.write_all(&mut raw_response).await?;
        debug!("Responded to {} with {:?}", writer.peer_addr()?, response);
    }
}

/// Server runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::formatted_timed_builder()
        .filter(None, log::LevelFilter::Debug)
        .init();

    let config: Config = confy::load("hdn-data-server", None)?;
    let db = Arc::new(sled::open(config.db_dir)?);
    let listener = TcpListener::bind(config.listener_addr).await?;
    info!("Listening on port {}", config.listener_addr.port());

    loop {
        let db = db.clone();
        let (socket, addr) = listener.accept().await?;
        debug!("Accepted connection from address {}", addr);
        tokio::spawn(async move {
            match connection_handler(socket, db).await {
                Ok(_) => debug!("Closed connection with {}", addr),
                Err(err) => debug!("Error occured while handling {}. Error: {}", addr, err),
            }
        });
    }
}
