
pub struct LinkerOptions {
	pub link_share_tag: bool,
	pub link_self: bool
}

impl LinkerOptions {
	pub fn new(link_share_tag: bool, link_self: bool) -> Self {
		LinkerOptions {
			link_share_tag,
			link_self
		}
	}
}

impl Default for LinkerOptions {
	fn default() -> Self {
		LinkerOptions {
			link_share_tag: false,
			link_self: false
		}
	}
}