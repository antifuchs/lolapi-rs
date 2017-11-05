pub mod by_champion;
use {dto, request, StatusCode};
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	app_limit: &'a Mutex<Option<GCRA>>,
	method_limits: &'a MethodLimits,
	summoner_id: i64,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: &'a K,
		app_limit: &'a Mutex<Option<GCRA>>,
		method_limits: &'a MethodLimits,
		summoner_id: i64,
	) -> Self {
		Self { region: region, key: key, app_limit: app_limit, method_limits: method_limits, summoner_id: summoner_id }
	}

	/// "Get all champion mastery entries sorted by number of champion points descending."
	///
	/// **Endpoint**: `/lol/champion-mastery/v3/champion-masteries/by-summoner/{summoner_id}`
	pub fn get(&self) -> Result<Vec<dto::ChampionMastery>, StatusCode> {
		let path = format!(
			"/lol/champion-mastery/v3/champion-masteries/by-summoner/{summoner_id}",
			summoner_id = self.summoner_id
		);
		request(self.region, self.key, &path, Some(&self.app_limit), &self.method_limits.get)
	}

	pub fn by_champion(&self, champion_id: i64) -> by_champion::Subclient<K> {
		by_champion::Subclient::new(
			self.region,
			self.key,
			&self.app_limit,
			&self.method_limits.by_champion,
			self.summoner_id,
			champion_id,
		)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

pub(super) struct MethodLimits {
	get: Mutex<Option<GCRA>>,
	by_champion: by_champion::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get: Mutex::default(), by_champion: by_champion::MethodLimits::new() }
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn get() {
		::CLIENT.champion_mastery_v3().champion_masteries().by_summoner(24338059).get().unwrap();
	}
}