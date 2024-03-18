use super::*;

pub fn visit_macro_punctuated_exprs<V>(v: &mut V, mac: &mut Macro)
where
	V: VisitMut
{
	macro_rules! try_parse {
		($parse:expr, $($visit:tt)+) => {
			#[allow(clippy::redundant_closure_call)]
			if let Ok(parsed) = mac.parse_body_with($parse) {
				let mut visit = $($visit)*;

				mac.tokens = visit(parsed);

				return;
			}
		}
	}

	try_parse!(
		Punctuated::<Expr, Token![,]>::parse_terminated,
		|mut exprs: Punctuated<Expr, Token![,]>| {
			for expr in &mut exprs {
				visit_expr_mut(v, expr);
			}

			exprs.to_token_stream()
		}
	);

	try_parse!(Block::parse_within, |mut stmts: Vec<Stmt>| {
		for stmt in &mut stmts {
			visit_stmt_mut(v, stmt);
		}

		quote! { #(#stmts)* }
	});
}
