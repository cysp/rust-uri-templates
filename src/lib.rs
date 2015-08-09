#![feature(rustc_private)]

#[macro_use] extern crate log;
extern crate automaton;

pub use types::{UriTemplate, UriTemplateOperator};
pub use types::{UriTemplateValues, UriTemplateValue};
pub use building::{UriTemplateBuilder, UriTemplateComponentBuilder, UriTemplateModifier};

mod types;
mod building;
pub mod parsing;

// pub mod uritemplate;
