use async_std::net::TcpListener;
use async_std::net::ToSocketAddrs;
use async_std::task;
use log::info;
use structopt::StructOpt;

use error::Result;

mod error;

#[derive(Debug, StructOpt)]
struct Config {
    #[structopt(short, long)]
    ip: Option<String>,

    #[structopt(short, long)]
    port: Option<String>,
}

fn main() {
    let config: Config = Config::from_args();
    simple_logger::init().unwrap();
    info!("Starting the server ......");

    let ip = config.ip.unwrap_or(String::from("127.0.0.1"));
    let port = config.port.unwrap_or(String::from("8080"));

    task::block_on(server(format!("{}:{}", ip, port)));
}

async fn server(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    Ok(())
}
