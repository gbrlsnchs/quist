mod app;
mod client;
mod utils;

use self::app::{App, Output};
use clap::Clap;
use std::io::{self, Result, Write};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
	let app = App::parse();
	let mut output = Output {
		stdout: io::stdout(),
		stderr: io::stderr(),
	};

	let (tx, rx) = flume::bounded(1);
	tokio::spawn(async move {
		signal::ctrl_c().await.unwrap();

		tx.send(())
	});

	let result = app.run(rx, &mut output).await;

	if let Err(err) = result {
		return writeln!(output.stderr, "{}: {}", utils::get_name(), err);
	}

	Ok(())
}
