mod prelude;
mod opcodes;
mod gen_opcodes;

use crate::prelude::*;
use tracing::{event, instrument, Level};

#[instrument]
fn main() -> Result<(), Error> {
    event!(Level::INFO, "Starting: {:?}", std::env::args().collect::<Vec<String>>());
    let _stdout_subscriber = tracing_subscriber::fmt::init();
    opcodes::generate()?;
    Ok(())
}

