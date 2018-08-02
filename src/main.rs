extern crate pnet;

mod frag;
use frag::ProxyFragmenter;

mod proxy_tx;
use proxy_tx::ProxyTxStruct;

use pnet::datalink;
use pnet::datalink::{NetworkInterface, EtherType, DataLinkSender, DataLinkReceiver, MacAddr};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket, EtherTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::transport::{transport_channel, TransportReceiver, TransportSender};
use pnet::transport::TransportChannelType;
use pnet::transport::TransportChannelType::{Layer3, Layer4};
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::{icmp_packet_iter, udp_packet_iter};

use std::thread;
use std::env;
use std::str::FromStr;
use std::net::Ipv4Addr;

const IPv4_ETHERTYPE: EtherType = 0x0800;

const GATEWAY_MAC_ADDR: MacAddr = MacAddr(0x0, 0x0, 0x0, 0x0, 0x0, 0x0);

// Make the buffer a single MTU
const BUF_SIZE: usize = 1500;

// Primarily interested in ICMP types 3 (unreachable) and 11 (TTL exceeded)
const ICMP_TYPE: usize = 11;

//const BRIDGED_IFACE_NAME: &'static str = "veth0";
const BRIDGED_IFACE_NAME: &'static str = "veth0";
const EXTERNAL_IFACE_NAME: &'static str = "lo";
const MIDDLEBOX_IP_ADDR_STR: &'static str = "192.168.1.1";
const DESTINATION_IP_ADDR_STR: &'static str = "192.168.1.2";


fn run_tx() {
    //thread::spawn(|| {
        let middlebox_ip = Ipv4Addr::from_str(MIDDLEBOX_IP_ADDR_STR).unwrap();
        let destination_ip = Ipv4Addr::from_str(DESTINATION_IP_ADDR_STR).unwrap();
        let mut proxy_tx = ProxyTxStruct::new(BRIDGED_IFACE_NAME,
                                              EXTERNAL_IFACE_NAME,
                                              middlebox_ip,
                                              destination_ip);
        proxy_tx.run();
    //});
}

fn main() {
    run_tx();

    /*
    thread::spawn(move || transmit_data(bridged_rx, ext_tx, outbound_iface_mac));
    thread::spawn(move || receive_data(bridged_tx));
    */
}
