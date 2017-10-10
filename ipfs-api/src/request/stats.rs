use request::ApiRequest;


pub struct StatsBitswap;

impl_skip_serialize!(StatsBitswap);

impl ApiRequest for StatsBitswap {
    #[inline]
    fn path() -> &'static str {
        "/stats/bitswap"
    }
}


pub struct StatsBw;

impl_skip_serialize!(StatsBw);

impl ApiRequest for StatsBw {
    #[inline]
    fn path() -> &'static str {
        "/stats/bw"
    }
}


pub struct StatsRepo;

impl_skip_serialize!(StatsRepo);

impl ApiRequest for StatsRepo {
    #[inline]
    fn path() -> &'static str {
        "/stats/repo"
    }
}
