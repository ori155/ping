use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid procotol")]
    InvalidProtocol,
    #[error("ICMP parsing error")]
    ICMPParsing {
        #[from]
        error: crate::packet::icmp::Error
    },
    #[error("IO error")]
    IO {
        #[from]
        error: std::io::Error
    },
    #[error("IPv4 error")]
    IPv4 {
        #[from]
        error: crate::packet::ipv4::Error
    },
}
