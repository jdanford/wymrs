#![cfg_attr(feature = "cargo-clippy", deny(clippy::all, clippy::pedantic))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::missing_docs_in_private_items, clippy::missing_errors_doc,)
)]

mod color;
mod direction;
mod error;
mod point;
mod tile;
mod world;
mod wyrm;

pub use color::{random_wyrm_color, Color};
pub use direction::{Direction, RelativeDirection};
pub use error::Result;
pub use point::Point;
pub use world::{NewWorldParams, World};
pub use wyrm::{NewWyrmParams, Wyrm};
