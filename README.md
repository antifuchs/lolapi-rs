# lolapi
Rate limited League of Legends API wrapper for Rust (WIP)

---

### Important

Rate limiting is implemented within `LolApiClient`, so you should only create one client per region, and you should reuse it instead of creating a new one for each request.

---

### Links

* [Crate](https://crates.io/crates/lolapi)
* [Documentation](https://docs.rs/lolapi)

### Currently supports

- [x] CHAMPION-MASTERY-V3
- [x] CHAMPION-V3
- [x] LEAGUE-V3
- [x] LOL-STATIC-DATA-V3
- [x] LOL-STATUS-V3
- [ ] MATCH-V3 (partial support)
- [ ] SPECTATOR-V3
- [ ] SUMMONER-V3
- [ ] TOURNAMENT-STUB-V3
- [ ] TOURNAMENT-V3
