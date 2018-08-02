/// This file implements the transmit/outbound portion of the proxy
/// using the fragment library. Note that this should be spawned in its own
/// thread, as it does not return control to the calling process.

use pnet::datalink;
use pnet::datalink::{NetworkInterface, MacAddr, DataLinkSender, DataLinkReceiver};
use pnet::datalink::Channel::Ethernet;

use pnet::packet::{Packet, MutablePacket, PacketSize};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket, EtherTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::MutableIpv4Packet;

use pnet::transport::{transport_channel, TransportReceiver, TransportSender};
use pnet::transport::TransportChannelType;

use std::net::Ipv4Addr;

// TODO: Make not a constant
const GATEWAY_MAC_ADDR: MacAddr = MacAddr(0x0, 0x0, 0x0, 0x0, 0x0, 0x0);

// TODO: Make traits
// TODO: Make cells or figure out mutability
pub struct ProxyTxStruct {
    in_iface: NetworkInterface,
    in_iface_channel: Box<DataLinkReceiver>,
    out_iface: NetworkInterface,
    out_iface_channel: Box<DataLinkSender>,
    next_hop_mac: MacAddr,
    middlebox_ip: Ipv4Addr,
    destination_ip: Ipv4Addr,
}

impl ProxyTxStruct {
    // TODO: Eventually, we should be able to construct ip addrs from the
    // interface objects, but this is currently a little complicated (I think)
    pub fn new(in_iface_name: &str,
               out_iface_name: &str,
               middlebox_ip: Ipv4Addr,
               destination_ip: Ipv4Addr) -> ProxyTxStruct {
        let in_iface = get_iface_from_name(in_iface_name);
        let out_iface = get_iface_from_name(out_iface_name);

        let (_, mut in_iface_channel) = establish_datalink_channel(&in_iface);
        let (mut out_iface_channel, _) = establish_datalink_channel(&out_iface);

        ProxyTxStruct {
            in_iface: in_iface,
            in_iface_channel: in_iface_channel,
            out_iface: out_iface,
            out_iface_channel: out_iface_channel,
            next_hop_mac: GATEWAY_MAC_ADDR,
            middlebox_ip: middlebox_ip,
            destination_ip: destination_ip,
        }
    }

    // TODO: Need to be mutable?
    // A: Probably - makes race conditions impossible
    pub fn set_middlebox_ip(&mut self, middlebox_ip: Ipv4Addr) {
        self.middlebox_ip = middlebox_ip;
    }

    pub fn set_destination_ip(&mut self, destination_ip: Ipv4Addr) {
        self.destination_ip = destination_ip;
    }

    pub fn run(&mut self) {
        loop {
            // Gets next Ethernet packet, or continue next iteration of loop
            let ethernet_packet = match self.in_iface_channel.next() {
                Ok(packet) => {
                    // We only care about valid Ethernet packets here;
                    // likewise, if new fails, we exit
                    match EthernetPacket::new(packet) {
                        Some(packet) => packet,
                        None => continue,
                    }
                },
                Err(e) => {
                    // Log and return none
                    continue;
                },
            };

            // ethernet_packet will now be valid and not None here

            // If it is not an Ipv4 packet, continue
            if ethernet_packet.get_ethertype() != EtherTypes::Ipv4 {
                continue;
            }

            // Mess with it and send it
            let mut mut_packet_vec = vec![ethernet_packet.packet_size(); 0];
        }
    }

    fn received_ethernet_packet(&mut self, ethernet_packet: EthernetPacket) {
        let offset = EthernetPacket::minimum_packet_size();
        let new_src_mac = self.out_iface.mac_address();
        let new_dst_mac = self.next_hop_mac;
        let new_dst_ip = self.middlebox_ip;
        self.out_iface_channel.build_and_send(1, ethernet_packet.packet().len(),
        &mut |mut new_packet| {
            let mut new_ethernet_packet = MutableEthernetPacket::new(new_packet).unwrap();
            new_ethernet_packet.clone_from(&ethernet_packet);
            new_ethernet_packet.set_source(new_src_mac);
            new_ethernet_packet.set_destination(new_dst_mac);
            let mut payload = new_ethernet_packet.payload_mut();
            let mut new_ip_packet = MutableIpv4Packet::new(payload).unwrap();
            let destination_ip = new_ip_packet.get_destination();
            new_ip_packet.set_source(destination_ip);
            new_ip_packet.set_destination(new_dst_ip);
        });
    }
}

// Utility functions
fn get_iface_from_name(iface_name: &str) -> NetworkInterface {
    let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;
    let interfaces = datalink::interfaces();

    // TODO: Fix panic
    let interface = interfaces.into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();
    interface
}

fn establish_transport_channel(protocol: TransportChannelType, buf_size: usize)
    -> (TransportSender, TransportReceiver) {
    let transport_channel = transport_channel(buf_size, protocol);
    match transport_channel {
        Ok((tx, mut rx)) => (tx, rx),
        Err(e) => panic!("Error: Failed to establish transport channel."),
    }
}

fn establish_datalink_channel(interface: &NetworkInterface)
        -> (Box<DataLinkSender>, Box<DataLinkReceiver>) {

    match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Error: Unknown datalink channel type"),
        Err(e) => panic!("Error: Unable to establish channel: {}", e),
    }
}
