/// Representation of possible authentication methods for the GitHub API.
#[derive(Debug, PartialEq)]
pub enum AuthMethod<'a> {
	/// Basic access authentication
	BasicAuth { username: &'a str, token: &'a str },
}
