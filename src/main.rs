//! main.rs

use zero2prod_hannukorvala::run;
use std::net::TcpListener;

// tokio::main is a procedural macro
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed binding to port 8000");
        
    run(listener)?.await
}
