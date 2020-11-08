pub const fn get_version() -> &'static str {
	if cfg!(debug_assertions) {
		"develop"
	} else {
		env!("CARGO_PKG_VERSION")
	}
}
