use crate::client::auth::AuthMethod;
use crate::client::data::{FileMap, Gist};
use crate::client::response::Response;
use crate::client::Client;
use crate::utils;
use clap::{Clap, ValueHint};
use flume::Receiver;
use futures::future;
use std::error::Error;
use std::io::Result as IoResult;
use std::io::Write;
use std::path::PathBuf;

pub struct Output<Stdout: Write, Stderr: Write> {
	pub stdout: Stdout,
	pub stderr: Stderr,
}

/// A CLI to create short-lived Gists.
#[derive(Clap, Debug, Default)]
#[clap(
	name = utils::get_name(),
	version = utils::get_version(),
	author = env!("CARGO_PKG_AUTHORS"),
)]
pub struct App {
	/// Credentials in basic access authentication format
	#[clap(long)]
	basic_auth: String,
	/// List of files to be included in the Gist
	#[clap(
		name = "FILE",
		required = true,
		parse(from_os_str),
		value_hint = ValueHint::AnyPath,
	)]
	files: Vec<PathBuf>,
}

impl App {
	/// Acts like an imperative shell and runs the application.
	pub async fn run<Stdout: Write, Stderr: Write>(
		mut self,
		exit: Receiver<()>,
		output: &mut Output<Stdout, Stderr>,
	) -> Result<(), Box<dyn Error>> {
		let Output {
			ref mut stdout,
			ref mut stderr,
		} = output;

		self.files.sort();
		let file_map: FileMap = self
			.read_files()
			.await?
			.iter()
			.map(|(path, content)| (*path, content.into()))
			.collect();

		let base_url = get_base_url();
		let client = Client::new(&base_url, self.parse_auth_method());

		let gist = Gist {
			description: None,
			files: file_map,
		};

		let response = client.create(&gist).await?;
		let gist;

		match response {
			Response::Ok(gist_created) => {
				write!(stderr, "URL created: ")?;
				write!(stdout, "{}", gist_created.url)?;
				gist = gist_created;
				writeln!(stderr)?;
			}
			Response::Err { message } => return Err(message.into()),
		};

		writeln!(
			stderr,
			"Waiting for termination in order to delete the Gist..."
		)?;
		exit.recv_async().await?;
		writeln!(stderr, "Gist {:?} successfully deleted! Bye.", gist.id)?;

		Ok(())
	}

	async fn read_files(&self) -> IoResult<Vec<(&str, Vec<u8>)>> {
		let files: Vec<_> = self.files.iter().map(tokio::fs::read).collect();
		let App {
			files: ref app_files,
			basic_auth: _,
		} = self;

		let files: Vec<_> = future::try_join_all(files)
			.await?
			.into_iter()
			.enumerate()
			.map(|(i, content)| (app_files[i].file_name().unwrap().to_str().unwrap(), content))
			.collect();

		Ok(files)
	}

	fn parse_auth_method(&self) -> AuthMethod {
		let credentials: Vec<_> = self.basic_auth.split(':').collect();

		match credentials[..] {
			[username, token] => AuthMethod::BasicAuth { username, token },
			// TODO: Handle lack of authentication.
			_ => todo!(),
		}
	}
}

fn get_base_url() -> String {
	#[cfg(test)]
	let base_url = mockito::server_url();
	#[cfg(not(test))]
	let base_url = "https://api.github.com".to_owned();

	base_url
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	use serde_json::json;
	use std::path::PathBuf;

	#[tokio::test]
	async fn test_files_are_correctly_read() -> IoResult<()> {
		let app = App {
			files: vec![PathBuf::from("test/foo.txt"), PathBuf::from("test/bar.txt")],
			..App::default()
		};

		let files = app.read_files().await?;

		assert_eq!(
			files,
			vec![
				("foo.txt", b"foo is here\n".to_vec()),
				("bar.txt", b"bar is here\n".to_vec()),
			],
		);

		Ok(())
	}

	#[test]
	fn test_basic_auth_is_correctly_parsed() {
		let app = App {
			basic_auth: String::from("foo:bar"),
			..App::default()
		};

		assert_eq!(
			app.parse_auth_method(),
			AuthMethod::BasicAuth {
				username: "foo",
				token: "bar",
			}
		);
	}

	#[test]
	#[should_panic]
	fn test_unimplemented_auth_panic() {
		let app = App::default();

		app.parse_auth_method();
	}

	#[tokio::test]
	async fn test_run() {
		let post_gists_mock = mockito::mock("POST", "/gists")
			.match_header("Accept", "application/vnd.github.v3+json")
			.match_header("Authorization", "Basic dXNlcm5hbWU6dG9rZW4=")
			.match_header("User-Agent", &*format!("quist/{}", utils::get_version()))
			.match_body(
				&*json!({
					"files": {
						"foo.txt": {
							"content": "foo is here\n"
						},
						"bar.txt": {
							"content": "bar is here\n"
						},
					}
				})
				.to_string(),
			)
			.with_body(
				json!({
					"url": "test://gist",
					"forks_url": "https://api.github.com/gists/aa5a315d61ae9438b18d/forks",
					"commits_url": "https://api.github.com/gists/aa5a315d61ae9438b18d/commits",
					"id": "aa5a315d61ae9438b18d",
					"node_id": "MDQ6R2lzdGFhNWEzMTVkNjFhZTk0MzhiMThk",
					"git_pull_url": "https://gist.github.com/aa5a315d61ae9438b18d.git",
					"git_push_url": "https://gist.github.com/aa5a315d61ae9438b18d.git",
					"html_url": "https://gist.github.com/aa5a315d61ae9438b18d",
					"created_at": "2010-04-14T02:15:15Z",
					"updated_at": "2011-06-20T11:34:15Z",
					"description": "Hello World Examples",
					"comments": 0,
					"comments_url": "https://api.github.com/gists/aa5a315d61ae9438b18d/comments/"
				})
				.to_string(),
			)
			.create();

		let app = App {
			basic_auth: String::from("username:token"),
			files: vec![PathBuf::from("test/foo.txt"), PathBuf::from("test/bar.txt")],
		};

		let mut output = Output {
			stdout: Vec::new(),
			stderr: Vec::new(),
		};

		let (tx, rx) = flume::bounded(1);
		tx.try_send(()).unwrap();
		let result = app.run(rx, &mut output).await;

		post_gists_mock.assert();

		assert!(result.is_ok());
		assert_eq!(output.stdout, b"test://gist");
	}
}
