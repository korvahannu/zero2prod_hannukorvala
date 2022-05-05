//! main.rs

use zero2prod_hannukorvala::run;

// tokio::main is a procedural macro
#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}