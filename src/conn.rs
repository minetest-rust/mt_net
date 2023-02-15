use super::PktInfo;
use delegate::delegate;
use mt_ser::{DefCfg, MtDeserialize, MtSerialize};
use std::{borrow::Cow, io};
use thiserror::Error;

pub trait Remote {
    type UdpSender: mt_rudp::UdpSender;
    type PktFrom: MtDeserialize;
    type PktTo: MtSerialize + PktInfo;
}

#[cfg(feature = "client")]
pub struct RemoteSrv;

#[cfg(feature = "client")]
impl Remote for RemoteSrv {
    type UdpSender = mt_rudp::ToSrv;
    type PktTo = crate::ToSrvPkt;
    type PktFrom = crate::ToCltPkt;
}

#[cfg(feature = "client")]
pub async fn connect(addr: &str) -> io::Result<(MtSender<RemoteSrv>, MtReceiver<RemoteSrv>)> {
    let (tx, rx) = mt_rudp::connect(addr).await?;
    Ok((MtSender(tx), MtReceiver(rx)))
}

/*

pub struct RemoteClt;
impl Remote for RemoteClt {
    type Sender = mt_rudp::ToClt;
    type To = crate::ToCltPkt;
    type From = crate::ToSrvPkt;
}

*/

pub struct MtSender<R: Remote>(pub mt_rudp::RudpSender<R::UdpSender>);
pub struct MtReceiver<R: Remote>(pub mt_rudp::RudpReceiver<R::UdpSender>);

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
        impl<R: Remote> $T<R> {
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

impl<R: Remote> MtReceiver<R> {
    pub async fn recv(&mut self) -> Option<Result<R::PktFrom, RecvError>> {
        self.0.recv().await.map(|res| {
            res.map_err(RecvError::from).and_then(|pkt| {
                // TODO: warn on trailing data
                R::PktFrom::mt_deserialize::<DefCfg>(&mut io::Cursor::new(pkt.data))
                    .map_err(RecvError::from)
            })
        })
    }
}

impl<R: Remote> MtSender<R> {
    pub async fn send(&self, pkt: &R::PktTo) -> Result<(), SendError> {
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
