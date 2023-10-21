use once_cell::sync::Lazy;
use palette::{FromColor, Oklch, Srgb};
use rand::Rng;
use rand_distr::Normal;
pub use sdl2::pixels::Color;

use crate::{tile, Result};

type NormalDistributionValues = (f32, f32);
type LchDistributionValues = (
    NormalDistributionValues,
    NormalDistributionValues,
    NormalDistributionValues,
);

pub static EMPTY: Lazy<Color> = Lazy::new(|| color_from_oklch(0.184, 0.052, 249.7));
pub static WALL: Lazy<Color> = Lazy::new(|| color_from_oklch(0.6, 0.09, 300.0));
pub static FOOD: Lazy<Color> = Lazy::new(|| color_from_oklch(0.4, 0.152, 19.7));
pub static MISSING: Lazy<Color> = Lazy::new(|| color_from_oklch(1.0, 0.0, 0.0));

// common
const ORANGEYELLOW: LchDistributionValues = ((0.8, 0.05), (0.25, 0.05), (90.0, 10.0));
const YELLOWGREEN: LchDistributionValues = ((0.85, 0.05), (0.3, 0.05), (120.0, 5.0));
const GREENGREEN: LchDistributionValues = ((0.8, 0.05), (0.25, 0.05), (145.0, 5.0));
const BLUEGREEN: LchDistributionValues = ((0.8, 0.05), (0.25, 0.05), (160.0, 5.0));
const CREAM: LchDistributionValues = ((0.85, 0.05), (0.1, 0.05), (80.0, 10.0));

// exotic
const LIGHTBLUE: LchDistributionValues = ((0.85, 0.05), (0.2, 0.05), (220.0, 5.0));
const FUCHSIA: LchDistributionValues = ((0.67, 0.05), (0.3, 0.02), (10.0, 10.0));

const WYRM_COLOR_DISTRIBUTIONS: [LchDistributionValues; 15] = [
    ORANGEYELLOW,
    ORANGEYELLOW,
    YELLOWGREEN,
    YELLOWGREEN,
    YELLOWGREEN,
    GREENGREEN,
    GREENGREEN,
    GREENGREEN,
    BLUEGREEN,
    BLUEGREEN,
    BLUEGREEN,
    CREAM,
    CREAM,
    LIGHTBLUE,
    FUCHSIA,
];

const HUE_MAX: f32 = 360.0;

fn random_normal<R: Rng>(rng: &mut R, (mean, std_dev): NormalDistributionValues) -> Result<f32> {
    let distribution = Normal::new(mean, std_dev).map_err(|e| e.to_string())?;
    Ok(rng.sample(distribution))
}

fn color_from_oklch(l: f32, chroma: f32, hue: f32) -> Color {
    let lch = Oklch::new(l, chroma, hue);
    let rgb: Srgb<u8> = Srgb::from_color(lch).into_format();
    let (r, g, b) = rgb.into_components();
    Color::RGB(r, g, b)
}

fn random_color<R: Rng>(rng: &mut R, (dl, dc, dh): LchDistributionValues) -> Result<Color> {
    let l = random_normal(rng, dl)?;
    let chroma = random_normal(rng, dc)?;
    let hue = (random_normal(rng, dh)? + HUE_MAX) % HUE_MAX;
    Ok(color_from_oklch(l, chroma, hue))
}

#[allow(clippy::module_name_repetitions)]
pub fn random_wyrm_color<R: Rng>(rng: &mut R, id: u16) -> Result<Color> {
    let n: i32 = WYRM_COLOR_DISTRIBUTIONS
        .len()
        .try_into()
        .map_err(|_| "too many color distributions".to_string())?;

    let offset: i32 = rng.gen_range(-1..=3);
    let base_index = i32::from(id - tile::WYRM);
    let raw_index = (base_index + offset + n) % n;
    let index = usize::try_from(raw_index).map_err(|_| format!("invalid index {raw_index}"))?;

    let dist_vals = WYRM_COLOR_DISTRIBUTIONS[index];
    random_color(rng, dist_vals)
}
