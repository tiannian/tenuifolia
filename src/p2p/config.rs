use libp2p::{
    core, dns, identity::Keypair, mplex, noise, tcp, uds, websocket, yamux, PeerId, Transport,
};

use crate::{PeerKeypair, Result};

pub enum TransportType {
    Websocket(Vec<String>),
    Uds(String),
}

pub struct Config {
    pub transport: TransportType,
    pub bootstrap_nodes: Vec<String>,
    pub timeout: u64,
    pub kademlia: KademliaConfig,
    pub re2: Re2Config,
}

pub struct KademliaConfig {
    pub enable_disjoint_query_paths: bool,
}

pub struct Re2Config {
    pub request_timeout: u64,
}

impl Config {
    fn build_websocket_transport(
        keypair: &Keypair,
        addrs: &[String],
    ) -> Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>> {
        let transport = {
            let mut tcp_ = tcp::TokioTcpTransport::new(tcp::GenTcpConfig::new().nodelay(true));

            for addr in addrs {
                let addr = addr.parse()?;
                tcp_.listen_on(addr)?;
            }

            let ws_dns_tcp = websocket::WsConfig::new(dns::TokioDnsConfig::system(tcp_)?);

            ws_dns_tcp
        };

        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(keypair)
            .expect("Signing libp2p-noise static DH keypair failed.");

        Ok(transport
            .upgrade(core::upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(core::upgrade::SelectUpgrade::new(
                yamux::YamuxConfig::default(),
                mplex::MplexConfig::default(),
            ))
            .timeout(std::time::Duration::from_secs(20))
            .boxed())
    }

    fn build_uds_transport(
        keypair: &Keypair,
        addr: &str,
    ) -> Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>> {
        let transport = {
            let mut uds = uds::UdsConfig::new();

            let addr = addr.parse()?;
            uds.listen_on(addr)?;

            uds
        };

        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(keypair)
            .expect("Signing libp2p-noise static DH keypair failed.");

        Ok(transport
            .upgrade(core::upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(core::upgrade::SelectUpgrade::new(
                yamux::YamuxConfig::default(),
                mplex::MplexConfig::default(),
            ))
            .timeout(std::time::Duration::from_secs(20))
            .boxed())
    }

    pub(crate) fn build_transport(
        &self,
        keypair: &PeerKeypair,
    ) -> Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>> {
        let kp = keypair.to_libp2p_keypair()?;

        match &self.transport {
            TransportType::Websocket(addrs) => Self::build_websocket_transport(&kp, addrs),
            TransportType::Uds(addr) => Self::build_uds_transport(&kp, addr),
        }
    }
}
