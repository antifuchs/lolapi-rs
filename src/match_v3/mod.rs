pub mod matches;
pub mod matchlists;
use ratelimit_meter::LeakyBucket;
use std::fmt::Display;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	app_limit: &'a mut Vec<LeakyBucket>,
	method_limits: &'a mut MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: &'a K,
		app_limit: &'a mut Vec<LeakyBucket>,
		method_limits: &'a mut MethodLimits,
	) -> Self {
		Self { region: region, key: key, app_limit: app_limit, method_limits: method_limits }
	}

	pub fn matches(&mut self) -> matches::Subclient<K> {
		matches::Subclient::new(self.region, self.key, self.app_limit, &mut self.method_limits.matches)
	}

	pub fn matchlists(&mut self) -> matchlists::Subclient<K> {
		matchlists::Subclient::new(self.region, self.key, self.app_limit, &mut self.method_limits.matchlists)
	}
}

pub(super) struct MethodLimits {
	matches: matches::MethodLimits,
	matchlists: matchlists::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { matches: matches::MethodLimits::new(), matchlists: matchlists::MethodLimits::new() }
	}
}
