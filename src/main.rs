//! main.rs

use std::net::TcpListener;
use zero2prod_hannukorvala::startup::run;
use zero2prod_hannukorvala::configuration::get_configuration;

// tokio::main is a procedural macro
#[tokio::main]
async fn main() -> std::io::Result<()> {


    let config = get_configuration()
        .expect("Failed to read configuration file");

    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(address).expect(format!("Failed binding to port {}", config.application_port).as_str());

    run(listener)?.await
}
