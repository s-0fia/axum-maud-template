#![allow(unused_imports)]
use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::mpsc::channel,
};
use tokio::{
    sync::Mutex,
    task::{self},
};
use utils::{timeprint, timeprintln};

use crate::pages::all_routes;

mod pages;

const LOCAL_HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const IP: IpAddr = /*LOCAL_HOST; */ IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
const PORT: u16 = 8080;
const SOCKET_ADDR: SocketAddr = SocketAddr::new(IP, PORT);

#[tokio::main]
async fn main() -> Result<()> {
    // Print the IP Address of the server
    timeprint!("Server Address: {SOCKET_ADDR}");

    if SOCKET_ADDR.ip() == LOCAL_HOST {
        println!(" (localhost:{PORT})");
    } else {
        println!();
    }

    let shutdown_handle = axum_server::Handle::new();

    let app = all_routes();

    axum_server::Server::bind(SOCKET_ADDR)
        .handle(shutdown_handle)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
