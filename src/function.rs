use super::*;

pub struct RemoveRefMut;

impl VisitMut for RemoveRefMut {
	fn visit_pat_ident_mut(&mut self, ident: &mut PatIdent) {
		visit_pat_ident_mut(self, ident);

		ident.by_ref.take();
		ident.mutability.take();
	}
}

pub fn get_args(sig: &Signature, include_receiver: bool) -> Punctuated<Expr, Token![,]> {
	let mut args = Punctuated::new();

	for arg in sig.inputs.iter() {
		match arg {
			FnArg::Typed(arg) => {
				let mut pat = arg.pat.as_ref().clone();

				RemoveRefMut {}.visit_pat_mut(&mut pat);

				args.push(parse_quote! { #pat });
			}

			FnArg::Receiver(rec) if include_receiver => {
				let token = &rec.self_token;

				args.push(parse_quote! { #token });
			}

			_ => {}
		}
	}

	args
}
