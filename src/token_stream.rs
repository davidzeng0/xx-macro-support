use super::*;

sealed_trait!(TokenStream);

pub trait TokenStreamExt: TokenStreamSealed {
	fn require_empty(&self) -> Result<()>;
}

impl TokenStreamSealed for TokenStream {}

impl TokenStreamExt for TokenStream {
	fn require_empty(&self) -> Result<()> {
		if self.is_empty() {
			Ok(())
		} else {
			Err(Error::new_spanned(self, "Unexpected tokens"))
		}
	}
}
