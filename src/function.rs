use super::*;

pub struct RemoveModifiers;

impl VisitMut for RemoveModifiers {
	fn visit_pat_ident_mut(&mut self, ident: &mut PatIdent) {
		visit_pat_ident_mut(self, ident);

		ident.by_ref.take();
		ident.mutability.take();
	}
}

#[must_use]
pub fn get_return_type(ret: &ReturnType) -> TokenStream {
	if let ReturnType::Type(_, ty) = ret {
		quote! { #ty }
	} else {
		quote! { () }
	}
}

#[must_use]
pub fn get_args(
	inputs: &Punctuated<FnArg, Token![,]>, include_receiver: bool
) -> Punctuated<Expr, Token![,]> {
	let mut args = Punctuated::new();

	for arg in inputs {
		match arg {
			FnArg::Typed(arg) => {
				let mut pat = arg.pat.as_ref().clone();

				RemoveModifiers {}.visit_pat_mut(&mut pat);

				args.push(parse_quote! { #pat });
			}

			FnArg::Receiver(rec) => {
				if include_receiver {
					let token = &rec.self_token;

					args.push(parse_quote! { #token });
				}
			}
		}
	}

	args
}
