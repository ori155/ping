use std::net::{SocketAddr, IpAddr};
use std::time::Duration;

use rand::random;
use socket2::{Domain, Protocol, Socket, Type};

use crate::errors::{Error};
use crate::packet::{EchoReply, EchoRequest, IpV4Packet, IcmpV4, IcmpV6, ICMP_HEADER_SIZE};

const TOKEN_SIZE: usize = 24;
const ECHO_REQUEST_BUFFER_SIZE: usize = ICMP_HEADER_SIZE + TOKEN_SIZE;
type Token = [u8; TOKEN_SIZE];

pub fn ping(addr: IpAddr, timeout: Option<Duration>, ttl: Option<u32>, ident: Option<u16>, seq_cnt: Option<u16>, payload: Option<&Token>) -> Result<EchoReply, Error> {
    let timeout = match timeout {
        Some(timeout) => Some(timeout),
        None => Some(Duration::from_secs(4)),
    };

    let dest = SocketAddr::new(addr, 0);
    let mut buffer = [0; ECHO_REQUEST_BUFFER_SIZE];

    let default_payload: &Token = &random();

    let request = EchoRequest {
        ident: ident.unwrap_or_else(random),
        seq_cnt: seq_cnt.unwrap_or(1),
        payload: payload.unwrap_or(default_payload),
    };

    let socket = if dest.is_ipv4() {
        request.encode::<IcmpV4>(&mut buffer[..])?;
        Socket::new(Domain::ipv4(), Type::raw(), Some(Protocol::icmpv4()))?
    } else {
        request.encode::<IcmpV6>(&mut buffer[..])?;
        Socket::new(Domain::ipv6(), Type::raw(), Some(Protocol::icmpv6()))?
    };

    socket.set_ttl(ttl.unwrap_or(64))?;

    socket.set_write_timeout(timeout)?;

    socket.send_to(&mut buffer, &dest.into())?;

    socket.set_read_timeout(timeout)?;

    let mut buffer: [u8; 2048] = [0; 2048];
    socket.recv_from(&mut buffer)?;

    let reply = if dest.is_ipv4() {
        let ipv4_packet = IpV4Packet::decode(&buffer)?;
        EchoReply::decode::<IcmpV4>(ipv4_packet.data)?
    } else {
        EchoReply::decode::<IcmpV6>(&buffer)?
    };

    Ok(reply)
}