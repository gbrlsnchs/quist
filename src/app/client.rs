use self::auth::AuthMethod;
use self::data::Gist;
use self::response::{GistCreated, GistDeleted, Response};
use crate::utils;
use reqwest::{Client as HttpClient, Error as HttpError};

pub mod auth;
pub mod data;
pub mod response;

/// Type alias to wrap a single Error from `reqwest`.
type HttpResult<T> = Result<T, HttpError>;

/// HTTP client for interacting with GitHub's API.
pub struct Client<'a> {
	base_url: &'a str,
	auth_method: AuthMethod<'a>,
	http_client: HttpClient,
}

impl<'a> Client<'a> {
	/// Constructs a new client with a fixed base URL and authentication method.
	pub fn new(base_url: &'a str, auth_method: AuthMethod<'a>) -> Client<'a> {
		Client {
			base_url,
			auth_method,
			http_client: HttpClient::new(),
		}
	}

	/// Creates a new Gist and returns its URL.
	pub async fn create(&self, gist: &Gist<'a>) -> HttpResult<Response<GistCreated>> {
		let base_url = format!("{}/gists", self.base_url);
		let mut request = self
			.http_client
			.post(&base_url)
			.header("Accept", "application/vnd.github.v3+json")
			.header("User-Agent", format!("quist/{}", utils::get_version()))
			.json(&gist);

		match self.auth_method {
			AuthMethod::BasicAuth { username, token } => {
				request = request.basic_auth(username, token.into());
			}
		}

		request.send().await?.json().await
	}

	/// Deletes an existing Gist.
	pub async fn delete(&self, gist_id: &str) -> HttpResult<Response<GistDeleted>> {
		let base_url = format!(
			"{base_url}/gists/{gist_id}",
			base_url = self.base_url,
			gist_id = gist_id,
		);
		let mut request = self
			.http_client
			.delete(&base_url)
			.header("Accept", "application/vnd.github.v3+json")
			.header("User-Agent", format!("quist/{}", utils::get_version()));

		match self.auth_method {
			AuthMethod::BasicAuth { username, token } => {
				request = request.basic_auth(username, token.into());
			}
		}

		let response = request.send().await?;

		match response.status().as_u16() {
			204 | 304 => Ok(Response::Ok(())),
			_ => response.json().await,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use data::File;
	use maplit::hashmap;
	use pretty_assertions::assert_eq;
	use serde_json::json;

	#[tokio::test]
	async fn test_create_gist() -> HttpResult<()> {
		let base_url = &mockito::server_url();
		let mock = mockito::mock("POST", "/gists")
			.match_header("Accept", "application/vnd.github.v3+json")
			.match_header("Authorization", "Basic dXNlcm5hbWU6dG9rZW4=")
			.match_header("User-Agent", &*format!("quist/{}", utils::get_version()))
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
			.with_status(201)
			.with_body(
				json!({
					"url": "https://api.github.com/gists/aa5a315d61ae9438b18d",
					"forks_url": "https://api.github.com/gists/aa5a315d61ae9438b18d/forks",
					"commits_url": "https://api.github.com/gists/aa5a315d61ae9438b18d/commits",
					"id": "aa5a315d61ae9438b18d",
					"node_id": "MDQ6R2lzdGFhNWEzMTVkNjFhZTk0MzhiMThk",
					"git_pull_url": "https://gist.github.com/aa5a315d61ae9438b18d.git",
					"git_push_url": "https://gist.github.com/aa5a315d61ae9438b18d.git",
					"html_url": "test://gist",
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
					content: String::from("Hello World\n"),
				},
			},
		};
		let response = client.create(&gist).await?;

		mock.assert();
		assert_eq!(
			response,
			Response::Ok(GistCreated {
				id: String::from("aa5a315d61ae9438b18d"),
				url: String::from("test://gist"),
			}),
		);

		Ok(())
	}

	#[tokio::test]
	async fn test_error_when_creating_gist() -> HttpResult<()> {
		let base_url = &mockito::server_url();
		let mock = mockito::mock("POST", "/gists")
			.match_header("Accept", "application/vnd.github.v3+json")
			.match_header("Authorization", "Basic dXNlcm5hbWU6dG9rZW4=")
			.match_header("User-Agent", &*format!("quist/{}", utils::get_version()))
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
			.with_status(401)
			.with_body(
				json!({
					"message": "needs auth",
					"documentation_url":
						"https://docs.github.com/rest/reference/gists#create-a-gist",
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
					content: String::from("Hello World\n"),
				},
			},
		};
		let response = client.create(&gist).await?;

		mock.assert();
		assert_eq!(
			response,
			Response::Err {
				message: String::from("needs auth"),
			},
		);

		Ok(())
	}

	#[tokio::test]
	async fn test_delete_gist() -> HttpResult<()> {
		let base_url = &mockito::server_url();
		let mock = mockito::mock("DELETE", "/gists/foo123")
			.match_header("Accept", "application/vnd.github.v3+json")
			.match_header("Authorization", "Basic dXNlcm5hbWU6dG9rZW4=")
			.match_header("User-Agent", &*format!("quist/{}", utils::get_version()))
			.with_status(204)
			.create();

		let client = Client::new(
			base_url,
			AuthMethod::BasicAuth {
				username: "username",
				token: "token",
			},
		);
		let response = client.delete("foo123").await?;

		mock.assert();
		assert_eq!(response, Response::Ok(()));

		Ok(())
	}

	#[tokio::test]
	async fn test_delete_gist_error() -> HttpResult<()> {
		let base_url = &mockito::server_url();
		let mock = mockito::mock("DELETE", "/gists/foo123")
			.match_header("Accept", "application/vnd.github.v3+json")
			.match_header("Authorization", "Basic dXNlcm5hbWU6dG9rZW4=")
			.match_header("User-Agent", &*format!("quist/{}", utils::get_version()))
			.with_status(403)
			.with_body(
				json!({
					"message": "needs auth",
					"documentation_url":
						"https://docs.github.com/rest/reference/gists#delete-a-gist",
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
		let response = client.delete("foo123").await?;

		mock.assert();
		assert_eq!(
			response,
			Response::Err {
				message: String::from("needs auth"),
			},
		);

		Ok(())
	}
}
