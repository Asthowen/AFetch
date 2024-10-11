#![allow(
    clippy::implicit_return,
    clippy::must_use_candidate,
    clippy::indexing_slicing,
    clippy::shadow_reuse,
    clippy::return_self_not_must_use,
    clippy::unwrap_used,
    clippy::exit,
    clippy::non_ascii_literal,
    clippy::arithmetic_side_effects,
    clippy::integer_division,
    clippy::exhaustive_structs,
    clippy::pub_use,
    clippy::mod_module_files,
    clippy::separated_literal_suffix,
    clippy::else_if_without_else
)]
#![deny(
    clippy::needless_return,
    clippy::uninlined_format_args,
    clippy::missing_const_for_fn,
    clippy::redundant_else,
    clippy::cloned_instead_of_copied,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::if_not_else,
    clippy::inefficient_to_string,
    clippy::option_as_ref_cloned,
    clippy::semicolon_if_nothing_returned,
    clippy::single_char_pattern,
    clippy::unused_async,
    clippy::suspicious_operation_groupings,
    clippy::useless_let_if_seq,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::fallible_impl_from,
    clippy::if_let_mutex,
    clippy::imprecise_flops,
    clippy::mutex_integer,
    clippy::string_to_string,
    clippy::string_add,
    clippy::ref_option_ref,
    clippy::use_self
)]

pub mod config;
pub mod logos;
pub mod system;
pub mod translations;
pub mod utils;

pub mod error;
