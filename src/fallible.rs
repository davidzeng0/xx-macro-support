use syn::parse::Parse;

use super::*;

pub fn try_expand<F>(func: F) -> TokenStream
where
	F: FnOnce() -> Result<TokenStream>
{
	match func() {
		Ok(tokens) => tokens,
		Err(err) => err.to_compile_error()
	}
}

#[must_use]
pub fn error_to_tokens<R: Parse>(error: Error) -> R {
	let error = error.to_compile_error();

	parse_quote! { #error }
}
