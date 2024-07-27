use std::mem::take;

use super::*;

sealed_trait!(WhereClause);

pub trait WhereClauseExt: WhereClauseSealed {
	fn default() -> Self;
}

impl WhereClauseSealed for WhereClause {}

impl WhereClauseExt for WhereClause {
	fn default() -> Self {
		Self {
			where_token: Default::default(),
			predicates: Punctuated::new()
		}
	}
}

sealed_trait!(Generics);

pub trait GenericsExt: GenericsSealed {
	fn remove_lifetimes(&mut self);

	fn to_types_turbofish(&self) -> TokenStream;
}

impl GenericsSealed for Generics {}

impl GenericsExt for Generics {
	fn remove_lifetimes(&mut self) {
		self.params = take(&mut self.params)
			.into_iter()
			.filter(|generic| !matches!(generic, GenericParam::Lifetime(_)))
			.collect();
	}

	fn to_types_turbofish(&self) -> TokenStream {
		let mut generics = self.clone();

		generics.remove_lifetimes();
		generics
			.split_for_impl()
			.1
			.as_turbofish()
			.into_token_stream()
	}
}
