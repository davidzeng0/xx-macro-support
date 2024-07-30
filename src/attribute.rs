use super::*;

sealed_trait!(Attributes);

pub trait AttributesExt: AttributesSealed {
	fn remove_if(&mut self, target: &str, kind_match: impl Fn(&Meta) -> bool) -> Option<Attribute>;

	fn remove_any(&mut self, target: &str) -> Option<Attribute>;

	fn remove_path(&mut self, target: &str) -> Option<Attribute>;

	fn remove_name_value(&mut self, target: &str) -> Option<MetaNameValue>;

	fn remove_list(&mut self, target: &str) -> Option<MetaList>;
}

impl AttributesSealed for Vec<Attribute> {}

impl AttributesExt for Vec<Attribute> {
	fn remove_if(&mut self, target: &str, kind_match: impl Fn(&Meta) -> bool) -> Option<Attribute> {
		let index = self.iter().position(|attr| {
			if !kind_match(&attr.meta) {
				return false;
			}

			attr.path().get_ident().is_some_and(|ident| ident == target)
		})?;

		Some(self.remove(index))
	}

	fn remove_any(&mut self, target: &str) -> Option<Attribute> {
		self.remove_if(target, |_| true)
	}

	fn remove_path(&mut self, target: &str) -> Option<Attribute> {
		self.remove_if(target, |meta| matches!(meta, Meta::Path(_)))
	}

	fn remove_name_value(&mut self, target: &str) -> Option<MetaNameValue> {
		let attr = self.remove_if(target, |meta| matches!(meta, Meta::NameValue(_)))?;

		Some(match attr.meta {
			Meta::NameValue(nv) => nv,
			_ => unreachable!()
		})
	}

	fn remove_list(&mut self, target: &str) -> Option<MetaList> {
		let attr = self.remove_if(target, |meta| matches!(meta, Meta::List(_)))?;

		Some(match attr.meta {
			Meta::List(list) => list,
			_ => unreachable!()
		})
	}
}
