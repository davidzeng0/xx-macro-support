use super::*;

#[macro_export]
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

	(($($tokens:tt)*)) => {
		$crate::macros::sealed_trait!($($tokens)*);
	}
}

pub use sealed_trait;

#[doc(hidden)]
#[macro_export]
macro_rules! attribute_macro_header {
	($ident:ident, $attr:ident, $item:ident, $block:block) => {
		#[proc_macro_attribute]
		pub fn $ident(
			$attr: proc_macro::TokenStream, $item: proc_macro::TokenStream
		) -> proc_macro::TokenStream $block
	}
}

#[doc(hidden)]
pub use attribute_macro_header;

#[macro_export]
macro_rules! declare_attribute_macro {
	{
		pub fn
		$ident:ident($attr:ident : TokenStream, $item:ident : TokenStream) -> TokenStream
		$block:block
	} => {
		$crate::macros::attribute_macro_header!($ident, $attr, $item, {
			fn $ident($attr: TokenStream, $item: TokenStream) -> TokenStream $block

			$ident($attr.into(), $item.into()).into()
		});
	};

	{
		pub fn
		$ident:ident($attr:ident : TokenStream, $item:ident : TokenStream) -> Result<TokenStream>
		$block:block
	} => {
		$crate::macros::attribute_macro_header!($ident, $attr, $item, {
			fn $ident($attr: TokenStream, $item: TokenStream) -> Result<TokenStream> $block

			$crate::fallible::try_expand(|| $ident($attr.into(), $item.into())).into()
		});
	};
}

pub use declare_attribute_macro;

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_header {
	($ident:ident, $item:ident, $block:block) => {
		#[proc_macro]
		pub fn $ident($item: proc_macro::TokenStream) -> proc_macro::TokenStream $block
	}
}

#[doc(hidden)]
pub use proc_macro_header;

#[doc(hidden)]
#[macro_export]
macro_rules! declare_proc_macro {
	{
		pub fn
		$ident:ident($item:ident : TokenStream) -> TokenStream
		$block:block
	} => {
		$crate::macros::proc_macro_header!($ident, $item, {
			fn $ident($item: TokenStream) -> TokenStream $block

			$ident($item.into()).into()
		});
	};

	{
		pub fn
		$ident:ident($item:ident : TokenStream) -> Result<TokenStream>
		$block:block
	} => {
		$crate::macros::proc_macro_header!($ident, $item, {
			fn $ident($item: TokenStream) -> Result<TokenStream> $block

			$crate::fallible::try_expand(|| $ident($item.into())).into()
		});
	};
}

pub use declare_proc_macro;
