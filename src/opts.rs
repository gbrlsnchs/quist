use crate::utils;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Default, StructOpt)]
#[structopt(
	name = utils::get_name(),
	version = utils::get_version(),
	author = env!("CARGO_PKG_AUTHORS"),
)]
pub struct Opts {
	/// Credentials in basic access authentication format
	#[structopt(long)]
	pub basic_auth: String,
	/// List of files to be included in the Gist
	#[structopt(name = "FILE", required = true, parse(from_os_str))]
	pub files: Vec<PathBuf>,
}
