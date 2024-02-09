use std::io::Read;

use tracing::{debug, error, info, span, trace, warn, Level};

fn main() {
    tracing_subscriber::fmt::init();
    let span = span!(Level::WARN, "main");
    let _guard = span.enter();
    for file in std::env::args() {

        let _guard = span.enter();

        info!(bytes = 0, "opening the file");

        let mut file = std::fs::File::open(file).unwrap();

        warn!("Reading file contents {:?}", file);

        let mut bytes = Vec::new();



        file.read(&mut bytes).unwrap();
        info!(bytes = ?bytes, "Parsing");


        info!("done with file");
    }
    info!("this happened!");
}
