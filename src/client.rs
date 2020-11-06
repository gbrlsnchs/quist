use reqwest::{Client as HttpClient, Error as HttpError};

mod response;
use response::GistCreated;

mod data;
pub use data::FileMap;
use data::Gist;

/// Type alias to wrap a single Error from `reqwest`.
type Result<T> = std::result::Result<T, HttpError>;

/// Representation of possible authentication methods for the GitHub API.
pub enum AuthMethod<'a> {
	/// Basic access authentication
	BasicAuth { username: &'a str, token: &'a str },
}

/// HTTP client for interacting with GitHub's API.
struct Client<'a> {
	base_url: &'a str,
	auth_method: AuthMethod<'a>,
	http_client: HttpClient,
}

impl<'a> Client<'a> {
	/// Constructs a new client with a fixed base URL and authentication method.
	fn new(base_url: &'a str, auth_method: AuthMethod<'a>) -> Client<'a> {
		Client {
			base_url,
			auth_method,
			http_client: HttpClient::new(),
		}
	}

	/// Creates a new Gist and returns its URL.
	async fn create(&self, gist: &Gist<'a>) -> Result<GistCreated> {
		let base_url = format!("{}/gists", self.base_url);
		let mut request = self
			.http_client
			.post(&base_url)
			.header("Accept", "application/vnd.github.v3+json")
			.json(&gist);

		match self.auth_method {
			AuthMethod::BasicAuth { username, token } => {
				request = request.basic_auth(username, Some(token));
			}
		}

		request.send().await?.json().await
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use maplit::hashmap;
	use pretty_assertions::assert_eq;
	use serde_json::json;

	use data::File;

	#[tokio::test]
	async fn test_create_gist() -> Result<()> {
		let base_url = &mockito::server_url();
		let mock = mockito::mock("POST", "/gists")
			.match_header("Accept", "application/vnd.github.v3+json")
			.match_header("Authorization", "Basic dXNlcm5hbWU6dG9rZW4=")
			.match_body(
				&*json!({
					"files": {
						"test": {
							"content": "Hello World\n"
						}
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

		let client = Client::new(
			base_url,
			AuthMethod::BasicAuth {
				username: "username",
				token: "token",
			},
		);
		let gist = Gist {
			description: None,
			files: hashmap! {
				"test" => File{
					content: "Hello World\n",
				},
			},
		};
		let response = client.create(&gist).await?;

		mock.assert();
		assert_eq!(
			response,
			GistCreated {
				url: String::from("test://gist")
			}
		);

		Ok(())
	}
}
