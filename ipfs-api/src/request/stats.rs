use request::ApiRequest;


pub struct StatsBitswap;

impl ApiRequest for StatsBitswap {
    #[inline]
    fn path() -> &'static str {
        "/stats/bitswap"
    }
}


pub struct StatsBw;

impl ApiRequest for StatsBw {
    #[inline]
    fn path() -> &'static str {
        "/stats/bw"
    }
}


pub struct StatsRepo;

impl ApiRequest for StatsRepo {
    #[inline]
    fn path() -> &'static str {
        "/stats/repo"
    }
}
