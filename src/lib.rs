#![cfg_attr(feature = "cargo-clippy", deny(clippy::all, clippy::pedantic))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::missing_docs_in_private_items,
        clippy::missing_errors_doc,
        clippy::missing_panics_doc
    )
)]

mod app;
mod color;
pub mod config;
mod direction;
mod tile;
mod world;
mod wyrm;

pub use app::App;
pub use color::{random_wyrm_color, Color};
pub use direction::{Direction, RelativeDirection};
pub use world::{NewWorldParams, World};
pub use wyrm::{NewWyrmParams, Wyrm};
