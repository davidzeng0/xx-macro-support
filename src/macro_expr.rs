use super::*;

pub fn visit_macro_punctuated_exprs<V>(v: &mut V, mac: &mut Macro)
where
	V: VisitMut
{
	if let Ok(mut exprs) =
		Punctuated::<Expr, Token![,]>::parse_terminated.parse2(mac.tokens.clone())
	{
		for expr in &mut exprs {
			visit_expr_mut(v, expr);
		}

		mac.tokens = exprs.to_token_stream();
	}
}
