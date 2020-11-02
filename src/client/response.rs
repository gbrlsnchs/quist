use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct GistCreated {
	pub url: String,
}
