use request::ApiRequest;


pub struct SwarmAddrsLocal;

impl ApiRequest for SwarmAddrsLocal {
    #[inline]
    fn path() -> &'static str {
        "/swarm/addrs/local"
    }
}


pub struct SwarmPeers;

impl ApiRequest for SwarmPeers {
    #[inline]
    fn path() -> &'static str {
        "/swarm/peers"
    }
}
