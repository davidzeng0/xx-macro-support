use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, visit_mut::*, *};

pub mod attribute;
pub mod fallible;
pub mod function;
pub mod visit_macro;
