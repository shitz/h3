use h3::{
    client::Connection,
    quic::{self, StreamId},
    Error,
};

use crate::{
    datagram::{Datagram, ReadDatagram},
    datagram_traits::HandleDatagramsExt,
    quic_traits::{self, RecvDatagramExt, SendDatagramExt},
};

impl<B, C> HandleDatagramsExt<C, B> for Connection<C, B>
where
    B: bytes::Buf,
    C: quic::Connection<B> + SendDatagramExt<B> + RecvDatagramExt,
    <C as quic_traits::RecvDatagramExt>::Error: h3::quic::Error + 'static,
    <C as quic_traits::SendDatagramExt<B>>::Error: h3::quic::Error + 'static,
{
    /// Sends a datagram
    fn send_datagram(&mut self, stream_id: StreamId, data: B) -> Result<(), Error> {
        self.inner
            .conn
            .send_datagram(Datagram::new(stream_id, data))?;
        Ok(())
    }

    /// Reads an incoming datagram
    fn read_datagram(&mut self) -> ReadDatagram<C, B> {
        ReadDatagram {
            conn: &mut self.inner,
            _marker: std::marker::PhantomData,
        }
    }
}
