use super::*;

pub fn remove_attr_kind(
	attrs: &mut Vec<Attribute>, target: &str, kind_match: impl Fn(&Meta) -> bool
) -> Option<Attribute> {
	let index = attrs.iter().position(|attr| {
		if !kind_match(&attr.meta) {
			return false;
		}

		attr.path().get_ident().is_some_and(|ident| ident == target)
	})?;

	Some(attrs.remove(index))
}

pub fn remove_attr_path(attrs: &mut Vec<Attribute>, target: &str) -> Option<Attribute> {
	remove_attr_kind(attrs, target, |meta| matches!(meta, Meta::Path(_)))
}

pub fn remove_attr_name_value(attrs: &mut Vec<Attribute>, target: &str) -> Option<Expr> {
	let attr = remove_attr_kind(attrs, target, |meta| matches!(meta, Meta::NameValue(_)))?;

	Some(match attr.meta {
		Meta::NameValue(nv) => nv.value,
		_ => unreachable!()
	})
}

pub fn remove_attr_list(attrs: &mut Vec<Attribute>, target: &str) -> Option<MetaList> {
	let attr = remove_attr_kind(attrs, target, |meta| matches!(meta, Meta::List(_)))?;

	Some(match attr.meta {
		Meta::List(list) => list,
		_ => unreachable!()
	})
}
