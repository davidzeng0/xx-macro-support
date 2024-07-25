use super::*;

macro_rules! sealed_trait {
	($($name:ident)?) => {
		$crate::paste::paste! {
			#[allow(non_snake_case)]
			mod [< __private_seal_ $($name)? >] {
				pub trait [< $($name)? Sealed >] {}
			}

			use [< __private_seal_ $($name)? >]::[< $($name)? Sealed >];
		}
	};

	(for $trait:ident) => {
		$crate::paste::paste! {
			#[allow(non_snake_case)]
			mod [< __private_seal_ $trait >] {
				pub trait [< $trait Sealed >]: super::$trait {}

				impl<T: super::$trait> [< $trait Sealed >] for T {}
			}

			use [< __private_seal_ $trait >]::[< $trait Sealed >];
		}
	};
}

sealed_trait!(Expr);

pub trait ExprExt: ExprSealed {
	fn as_lit_str(&self) -> Result<&LitStr>;

	fn as_lit_str_mut(&mut self) -> Result<&mut LitStr>;
}

impl ExprSealed for Expr {}

macro_rules! as_lit_str_impl {
	($self:ident) => {
		match $self {
			Self::Lit(ExprLit { lit: Lit::Str(str), .. }) => Ok(str),
			_ => Err(Error::new_spanned($self, "Expected a string literal"))
		}
	};
}

impl ExprExt for Expr {
	fn as_lit_str(&self) -> Result<&LitStr> {
		as_lit_str_impl!(self)
	}

	fn as_lit_str_mut(&mut self) -> Result<&mut LitStr> {
		as_lit_str_impl!(self)
	}
}
