mod app;
mod client;
mod utils;

use self::app::{App, Output};
use clap::Clap;
use std::io::{self, Result, Write};

#[tokio::main]
async fn main() -> Result<()> {
	let app = App::parse();
	let mut output = Output {
		stdout: io::stdout(),
		stderr: io::stderr(),
	};

	let result = app.run(&mut output).await;

	if let Err(err) = result {
		return writeln!(output.stderr, "{}: {}", utils::get_name(), err);
	}

	Ok(())
}
