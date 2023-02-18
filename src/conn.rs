use super::PktInfo;
use delegate::delegate;
use mt_ser::{DefCfg, MtDeserialize, MtSerialize};
use std::{borrow::Cow, io};
use thiserror::Error;

pub trait Peer {
    type UdpPeer: mt_rudp::UdpPeer;
    type PktFrom: MtDeserialize;
    type PktTo: MtSerialize + PktInfo;
}

#[cfg(feature = "client")]
pub struct RemoteSrv;

#[cfg(feature = "client")]
impl Peer for RemoteSrv {
    type UdpPeer = mt_rudp::RemoteSrv;
    type PktTo = crate::ToSrvPkt;
    type PktFrom = crate::ToCltPkt;
}

#[cfg(feature = "client")]
pub async fn connect(addr: &str) -> io::Result<(MtSender<RemoteSrv>, MtReceiver<RemoteSrv>)> {
    let (tx, rx) = mt_rudp::connect(addr).await?;
    Ok((MtSender(tx), MtReceiver(rx)))
}

/*

#[cfg(feature = "server")]
pub struct RemoteClt;

#[cfg(feature = "server")]
impl Peer for RemoteClt {
    type UdpPeer = mt_rudp::RemoteClt;
    type To = crate::ToCltPkt;
    type From = crate::ToSrvPkt;
}

*/

pub struct MtSender<P: Peer>(pub mt_rudp::RudpSender<P::UdpPeer>);
pub struct MtReceiver<P: Peer>(pub mt_rudp::RudpReceiver<P::UdpPeer>);

#[derive(Error, Debug)]
pub enum RecvError {
    #[error("connection error: {0}")]
    ConnError(#[from] mt_rudp::Error),
    #[error("deserialize error: {0}")]
    DeserializeError(#[from] mt_ser::DeserializeError),
}

#[derive(Error, Debug)]
pub enum SendError {
    #[error("connection error: {0}")]
    ConnError(#[from] io::Error),
    #[error("serialize error: {0}")]
    SerializeError(#[from] mt_ser::SerializeError),
}

macro_rules! impl_delegate {
    ($T:ident) => {
        impl<P: Peer> $T<P> {
            delegate! {
                to self.0 {
                    pub async fn peer_id(&self) -> u16;
                    pub async fn is_server(&self) -> bool;
                    pub async fn close(self);
                }
            }
        }
    };
}

impl_delegate!(MtSender);
impl_delegate!(MtReceiver);

impl<P: Peer> MtReceiver<P> {
    pub async fn recv(&mut self) -> Option<Result<P::PktFrom, RecvError>> {
        self.0.recv().await.map(|res| {
            res.map_err(RecvError::from).and_then(|pkt| {
                // TODO: warn on trailing data
                P::PktFrom::mt_deserialize::<DefCfg>(&mut io::Cursor::new(pkt.data))
                    .map_err(RecvError::from)
            })
        })
    }
}

impl<P: Peer> MtSender<P> {
    pub async fn send(&self, pkt: &P::PktTo) -> Result<(), SendError> {
        let mut writer = Vec::new();
        pkt.mt_serialize::<DefCfg>(&mut writer)?;

        let (chan, unrel) = pkt.pkt_info();
        self.0
            .send(mt_rudp::Pkt {
                chan,
                unrel,
                data: Cow::Borrowed(&writer),
            })
            .await?;

        Ok(())
    }
}

// derive(Clone) adds unwanted trait bound to P
impl<P: Peer> Clone for MtSender<P> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
