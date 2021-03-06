use {dto, request_with_query, Locale, StatusCode};
use ratelimit_meter::LeakyBucket;
use std::fmt::Display;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	method_limits: &'a mut MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(region: &'static str, key: &'a K, method_limits: &'a mut MethodLimits) -> Self {
		Self { region: region, key: key, method_limits: method_limits }
	}

	/// "Retrieve map data"
	///
	/// **Endpoint**: `/lol/static-data/v3/maps`
	pub fn get(&mut self, locale: Option<Locale>, version: Option<&str>) -> Result<dto::MapData, StatusCode> {
		let path = "/lol/static-data/v3/maps";

		let mut params = vec![];
		if let Some(locale) = locale {
			params.push(("locale", locale.to_str()));
		}
		if let Some(version) = version {
			params.push(("version", version));
		}

		request_with_query(self.region, self.key, path, params, &mut vec![], &mut self.method_limits.get)
	}
}

pub(super) struct MethodLimits {
	get: Vec<LeakyBucket>,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get: vec![] }
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn get() {
		::CLIENT.lock().unwrap().static_data_v3().maps().get(Some(::Locale::en_US), None).unwrap();
	}
}
