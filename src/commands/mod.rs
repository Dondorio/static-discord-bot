// pub mod ping;
use crate::Data;

pub mod changelog;
pub mod ping;
pub mod sermon;
pub mod static_image;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
