use std::net::IpAddr;
use pnet::{
	packet::{
		ip::IpNextHeaderProtocols,
		tcp::{MutableTcpPacket, TcpFlags}
	},
	transport::{tcp_packet_iter, transport_channel, TransportChannelType, TransportProtocol}
};

const CODE: u32 = 0x12340987;
const PORT: u16 = 443;

fn main() {
	let (mut tx, mut rx) = match transport_channel(
		4096,
		TransportChannelType::Layer4(TransportProtocol::Ipv6(IpNextHeaderProtocols::Tcp))
	) {
		Ok(x) => x,
		Err(e) => panic!("Failed to create channel: {:?}", e)
	};
	let mut rx_iter = tcp_packet_iter(&mut rx);

	let mut buf = [0u8; 20];
	let mut tcp_rst = MutableTcpPacket::new(&mut buf[..]).unwrap();
	tcp_rst.set_source(PORT);
	tcp_rst.set_data_offset(20 / 4);
	tcp_rst.set_flags(TcpFlags::RST | TcpFlags::ACK);
	tcp_rst.set_sequence(0);
	tcp_rst.set_acknowledgement(CODE);
	tcp_rst.set_window(0);

	println!("Hongbao 2025 is ready.");
	loop {
		match rx_iter.next() {
			Ok((packet, addr)) => {
				if packet.get_destination() != PORT {
					continue;
				}
				if packet.get_flags() != TcpFlags::SYN {
					continue;
				}
				let _addr6 = match addr {
					IpAddr::V6(x) => x,
					_ => continue
				};
				println!("Received TCP SYN from [{}]:{}", addr, packet.get_source());
				tcp_rst.set_destination(packet.get_source());
				// tcp_rst.set_checksum(ipv6_checksum(&tcp_rst.to_immutable(), &addr6, &addr6));
				if let Err(e) = tx.send_to(&tcp_rst, addr) {
					eprintln!("Failed to send reply to {}: {}", addr, e)
				}
			},
			Err(e) => panic!("Failed to receive: {:?}", e)
		}
	}
}
