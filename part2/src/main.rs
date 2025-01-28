use std::net::{IpAddr, Ipv6Addr};
use pnet::{
	packet::{
		ip::IpNextHeaderProtocols,
		tcp::{MutableTcpPacket, TcpFlags, ipv6_checksum}
	},
	transport::{tcp_packet_iter, transport_channel, TransportChannelType, TransportProtocol}
};

const CODE: u32 = 37085499;
const PORT: u16 = 443;
const HOST: Ipv6Addr = Ipv6Addr::from_bits(0x2402_4e00_1801_ef0c_0000_9e11_82c3_2531u128);

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
	tcp_rst.set_sequence(CODE);
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
				let addr6 = match addr {
					IpAddr::V6(x) => x,
					_ => continue
				};
				println!("Received TCP SYN from [{}]:{}", addr, packet.get_source());
				tcp_rst.set_destination(packet.get_source());
				tcp_rst.set_acknowledgement(packet.get_sequence() + 1);
				tcp_rst.set_checksum(ipv6_checksum(&tcp_rst.to_immutable(), &HOST, &addr6));
				if let Err(e) = tx.send_to(&tcp_rst, addr) {
					eprintln!("Failed to send reply to {}: {}", addr, e)
				}
			},
			Err(e) => panic!("Failed to receive: {:?}", e)
		}
	}
}
