use tracing::{Level, info};

mod state;
mod camera;
mod assets;
mod model;
mod texture;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("starting atir renderer");

    pollster::block_on(state::run());
}
