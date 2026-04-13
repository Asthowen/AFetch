#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]

pub mod config;
pub mod error;
pub mod logos;
pub mod system;
pub mod translations;
pub mod util;
