use serde::{Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};

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
	#[serde(serialize_with = "ordered_map")]
	pub files: FileMap<'a>,
}

fn ordered_map<'a, S>(value: &FileMap<'a>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let ordered: BTreeMap<_, _> = value.iter().collect();

	ordered.serialize(serializer)
}
