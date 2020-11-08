use std::collections::HashMap;

use serde::Serialize;

/// Represents a file to be uploaded to GitHub as part of a Gist.
#[derive(Serialize)]
pub struct File {
	pub content: String,
}

impl Into<File> for &Vec<u8> {
	fn into(self) -> File {
		File {
			content: String::from_utf8_lossy(self).to_string(),
		}
	}
}

/// Type for the list of files in a Gist.
pub type FileMap<'a> = HashMap<&'a str, File>;

/// Gist object.
#[derive(Serialize)]
pub struct Gist<'a> {
	/// Describes the Gist
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<&'a str>,
	/// List of files to be uploaded in a Gist
	pub files: FileMap<'a>,
}
