use self::quist::{utils, App};
use std::env;
use structopt::clap::Shell;
use structopt::StructOpt;

#[allow(dead_code)]
#[path = "src/lib.rs"]
mod quist;

fn main() {
	let outdir = match env::var_os("OUT_DIR") {
		None => return,
		Some(outdir) => outdir,
	};

	let shells = vec![Shell::Bash, Shell::Fish, Shell::Zsh];

	for sh in shells {
		App::clap().gen_completions(utils::get_name(), sh, &outdir);
	}
}
