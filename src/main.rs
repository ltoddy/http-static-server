use std::error;
use std::future::Future;

use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use log::info;
use log::Level;
use structopt::StructOpt;

mod method;
mod request;

#[derive(Debug, StructOpt)]
struct Config {
    #[structopt(short, long)]
    ip: Option<String>,

    #[structopt(short, long)]
    port: Option<String>,
}

fn main() {
    let default_ip = String::from("127.0.0.1");
    let default_port = String::from("8080");

    let config: Config = Config::from_args();
    simple_logger::init_with_level(Level::Info).unwrap();
    info!("Starting the server ......");

    let ip = config.ip.unwrap_or(default_ip);
    let port = config.port.unwrap_or(default_port);

    task::block_on(serve(ip, port)).expect("server failed to start")
}

async fn serve(ip: String, port: String) -> Result<(), Box<dyn error::Error>> {
    let addrs = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(addrs).await?;
    info!("Server run on: {}:{}", ip, port);

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        info!("Accept from: {}", stream.peer_addr()?);
        spawn_and_log_error(accept_connection(stream));
    }

    Ok(())
}

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<(), Box<dyn error::Error>>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}

async fn accept_connection(mut stream: TcpStream) -> Result<(), Box<dyn error::Error>> {
    let mut buffer = [0u8; 4096];

    stream.read(&mut buffer).await?;

    Ok(())
}
