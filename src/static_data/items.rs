use {dto, request_with_query, Locale, StaticDataItemTags, StatusCode};
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K> {
	region: &'static str,
	key: K,
	method_limits: &'a MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(region: &'static str, key: K, method_limits: &'a MethodLimits) -> Self {
		Self { region: region, key: key, method_limits: method_limits }
	}

	/// "Retrieves item list"
	///
	/// **Endpoint**: `/lol/static-data/v3/items`
	pub fn get(
		&self,
		locale: Option<Locale>,
		version: Option<&str>,
		tags: &StaticDataItemTags,
	) -> Result<dto::ItemList, StatusCode> {
		let path = "/lol/static-data/v3/items";

		let mut params = vec![];
		if let Some(locale) = locale {
			params.push(("locale", locale.to_str()));
		}
		if let Some(version) = version {
			params.push(("version", version));
		}
		let params = params.into_iter().chain(tags.to_query_pairs().into_iter());

		request_with_query(self.region, &self.key, path, params, None, &self.method_limits.get)
	}

	/// "Retrieves item by ID"
	///
	/// `tags.groups` and `tags.tree` are ignored.
	///
	/// **Endpoint**: `/lol/static-data/v3/items/{id}`
	pub fn get_id(
		&self,
		id: i32,
		locale: Option<Locale>,
		version: Option<&str>,
		tags: &StaticDataItemTags,
	) -> Result<dto::Item, StatusCode> {
		let mut tags = *tags;
		tags.groups = false;
		tags.tree = false;

		let path = format!("/lol/static-data/v3/items/{id}", id = id);

		let mut params = vec![];
		if let Some(locale) = locale {
			params.push(("locale", locale.to_str()));
		}
		if let Some(version) = version {
			params.push(("version", version));
		}
		let params = params.into_iter().chain(tags.to_query_pairs().into_iter());

		request_with_query(self.region, &self.key, &path, params, None, &self.method_limits.get_id)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

pub(super) struct MethodLimits {
	get: Mutex<Option<GCRA>>,
	get_id: Mutex<Option<GCRA>>,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get: Mutex::default(), get_id: Mutex::default() }
	}
}