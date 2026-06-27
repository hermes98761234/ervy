pub mod color;
pub mod datum;
pub mod options;

pub mod bar;
pub mod bullet;
pub mod donut;
pub mod gauge;
pub mod pie;
pub mod scatter;

pub use bar::bar;
pub use bullet::bullet;
pub use color::{bg, fg, Color, Style};
pub use datum::{Datum, DatumValue};
pub use donut::donut;
pub use gauge::gauge;
pub use options::{
    BarOptions, BulletOptions, DonutOptions, GaugeOptions, PieOptions, ScatterOptions,
};
pub use pie::pie;
pub use scatter::scatter;
