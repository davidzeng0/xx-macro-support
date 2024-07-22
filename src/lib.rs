#![allow(clippy::wildcard_imports)]

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::visit_mut::*;
use syn::*;

pub mod attribute;
pub mod fallible;
pub mod function;
pub mod visit_macro;
