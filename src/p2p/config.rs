use std::io;

use libp2p::{core, dns, identity::Keypair, mplex, noise, tcp, websocket, yamux, PeerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Transport {
    Tcp,
    Websocket,
    Uds,
}

impl Transport {
    fn build_tcp_transport(
        keypair: &Keypair,
    ) -> io::Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>> {
        use libp2p::Transport;

        let transport = {
            let dns_tcp = dns::TokioDnsConfig::system(tcp::TokioTcpTransport::new(
                tcp::GenTcpConfig::new().nodelay(true),
            ))?;
            let ws_dns_tcp = websocket::WsConfig::new(dns::TokioDnsConfig::system(
                tcp::TokioTcpTransport::new(tcp::GenTcpConfig::new().nodelay(true)),
            )?);
            dns_tcp.or_transport(ws_dns_tcp)
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

    fn build_websocket_transport(
        keypair: &Keypair,
    ) -> io::Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>> {
        use libp2p::Transport;

        let transport = {
            let dns_tcp = dns::TokioDnsConfig::system(tcp::TokioTcpTransport::new(
                tcp::GenTcpConfig::new().nodelay(true),
            ))?;
            let ws_dns_tcp = websocket::WsConfig::new(dns::TokioDnsConfig::system(
                tcp::TokioTcpTransport::new(tcp::GenTcpConfig::new().nodelay(true)),
            )?);
            dns_tcp.or_transport(ws_dns_tcp)
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
        keypair: &Keypair,
    ) -> io::Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>> {
        match self {
            Self::Tcp => Self::build_tcp_transport(keypair),
            Self::Websocket => Self::build_tcp_transport(keypair),
            Self::Uds => Self::build_tcp_transport(keypair),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub transport: Vec<Transport>,
    pub bootstrap_nodes: Vec<String>,
    pub peer_key: PeerKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerKey {
    pub peer_id: String,
    pub public_key: String,
    pub secret_key: String,
}
