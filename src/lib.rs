#![allow(
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,
    clippy::separated_literal_suffix,
    clippy::missing_inline_in_public_items,
    clippy::non_ascii_literal,
    clippy::must_use_candidate,
    clippy::mod_module_files,
    clippy::else_if_without_else,
    clippy::unused_self,
    clippy::cast_precision_loss
)]
#![deny(clippy::needless_return, clippy::str_to_string)]

pub mod config;
pub mod logos;
pub mod system;
pub mod translations;
pub mod utils;

pub mod error;
