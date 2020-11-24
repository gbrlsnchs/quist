use serde::{self, Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct GistCreated {
	pub url: String,
}

pub type GistDeleted = ();

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Response<T> {
	Ok(T),
	Err { message: String },
}
