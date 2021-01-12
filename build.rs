use opts::Opts;
use std::env;
use structopt::clap::Shell;
use structopt::StructOpt;

#[path = "src/opts.rs"]
mod opts;

#[path = "src/utils.rs"]
mod utils;

fn main() {
	let outdir = match env::var_os("OUT_DIR") {
		None => return,
		Some(outdir) => outdir,
	};

	let shells = vec![Shell::Bash, Shell::Fish, Shell::Zsh];

	for sh in shells {
		Opts::clap().gen_completions(utils::get_name(), sh, &outdir);
	}
}
