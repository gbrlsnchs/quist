use serde::{self, Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct GistCreated {
	pub id: String,
	#[serde(rename(deserialize = "html_url"))]
	pub url: String,
}

pub type GistDeleted = ();

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Response<T> {
	Ok(T),
	Err { message: String },
}
