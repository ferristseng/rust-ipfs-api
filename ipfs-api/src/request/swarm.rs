use request::ApiRequest;


pub struct SwarmAddrsLocal;

impl_skip_serialize!(SwarmAddrsLocal);

impl ApiRequest for SwarmAddrsLocal {
    #[inline]
    fn path() -> &'static str {
        "/swarm/addrs/local"
    }
}


pub struct SwarmPeers;

impl_skip_serialize!(SwarmPeers);

impl ApiRequest for SwarmPeers {
    #[inline]
    fn path() -> &'static str {
        "/swarm/peers"
    }
}
