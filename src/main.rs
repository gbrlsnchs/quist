mod app;
mod client;
mod utils;

use self::app::{App, Output};
use clap::Clap;
use std::io::{self, Error as IoError, ErrorKind as IoErrorKind, Result as IoResult, Write};
use tokio::signal;
use tokio::sync::mpsc::{self, Sender};

#[tokio::main]
async fn main() -> IoResult<()> {
	let app = App::parse();
	let mut output = Output {
		stdout: io::stdout(),
		stderr: io::stderr(),
	};

	let (tx, rx) = mpsc::channel(1);

	tokio::spawn(async move { handle_ctrlc(tx).await });

	let result = app.run(rx, &mut output).await;
	if let Err(err) = result {
		return writeln!(output.stderr, "{}: {}", utils::get_name(), err);
	}

	Ok(())
}

async fn handle_ctrlc(mut tx: Sender<()>) -> IoResult<()> {
	signal::ctrl_c().await?;

	if tx.send(()).await.is_err() {
		return Err(IoError::new(IoErrorKind::Other, "could not send signal"));
	}

	Ok(())
}
