extern crate soto;
#[macro_use] extern crate slog;
extern crate slog_term;

use slog::DrainExt;

fn main() {
    // Initialize logging
    let drain = slog_term::streamer()
        .use_custom_timestamp(|_| Ok(()))
        .compact()
        .build().fuse();
    let log = slog::Logger::root(drain, o!());
    info!(log, "Running build using soto {}", env!("CARGO_PKG_VERSION"));

    // Run actual build
    match soto::build(&log, "./") {
        Ok(_) => {},
        Err(e) => println!("Soto error: {}", e),
    }
}
