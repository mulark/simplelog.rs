use log::*;
use simplelog::*;

use std::fs::File;

fn main() {
    CombinedLogger::init(vec![
        #[cfg(feature = "term")]
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap(),
        #[cfg(not(feature = "term"))]
        SimpleLogger::new(LevelFilter::Warn, Config::default()),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("my_rust_binary.log").unwrap(),
        ),
    ])
    .unwrap();

    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");
}
