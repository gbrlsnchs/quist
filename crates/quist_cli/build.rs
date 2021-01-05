use clap::IntoApp;
use clap_generate::generators::{Bash, Fish, Zsh};
use quist::{utils, App};
use std::env;

fn main() {
	let outdir = match env::var_os("OUT_DIR") {
		None => return,
		Some(outdir) => outdir,
	};

	let mut app = App::into_app();
	clap_generate::generate_to::<Bash, _, _>(&mut app, utils::get_name(), &outdir);
	clap_generate::generate_to::<Fish, _, _>(&mut app, utils::get_name(), &outdir);
	clap_generate::generate_to::<Zsh, _, _>(&mut app, utils::get_name(), &outdir);
}
