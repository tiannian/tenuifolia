use std::io;

use async_trait::async_trait;
use libp2p::{
    core::ProtocolName,
    futures::{AsyncRead, AsyncWrite},
    request_response::RequestResponseCodec,
};

use crate::message;

#[derive(Debug, Clone)]
pub struct Protocol {}

impl ProtocolName for Protocol {
    fn protocol_name(&self) -> &[u8] {
        "/tenuifolia/re".as_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct Codec {}

#[async_trait]
impl RequestResponseCodec for Codec {
    type Protocol = Protocol;

    type Request = message::Message;

    type Response = message::Message;

    async fn read_request<T>(&mut self, _: &Protocol, io: &mut T) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        Ok(message::Message::PublishBlock)
    }

    async fn read_response<T>(&mut self, _: &Protocol, io: &mut T) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        Ok(message::Message::PublishBlock)
    }

    async fn write_request<T>(
        &mut self,
        _: &Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _: &Protocol,
        io: &mut T,
        req: Self::Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        Ok(())
    }
}
