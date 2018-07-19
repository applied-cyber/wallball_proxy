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

const BRIDGED_IFACE_NAME: &'static str = "veth0";
const EXTERNAL_IFACE_NAME: &'static str = "eth0";
const PROXY_IP_ADDR_STR: &'static str = "192.168.1.1";

fn receive_data(outbound_iface: Box<DataLinkSender>) {
    /*
    let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Icmp));
    let transport_channel = transport_channel(BUF_SIZE, protocol);

    match transport_channel {
        Ok((tx, mut rx)) => {
            let mut icmp_iterator = icmp_packet_iter(&mut rx);
            loop {
                match icmp_iterator.next() {
                    Ok((packet, _ip_addr)) => {
                        println!("Received {:?}", packet.get_icmp_type());
                        println!("Payload: {:?}", packet.payload());
                    },
                    Err(e) => {
                        panic!("An error occured while reading");
                    },
                }
            }
        },
        Err(e) => {
            panic!("ERROR: Failed to establish transport channel.");
        },
    }
    */
}

fn transmit_data(mut inbound_iface: Box<DataLinkReceiver>,
                 mut outbound_iface: Box<DataLinkSender>,
                 outbound_iface_mac: MacAddr) {

    let proxy_ip_addr = Ipv4Addr::from_str(PROXY_IP_ADDR_STR).unwrap();

    // Listen on bridged interface, then rewrite packet and forward to PROXY_IP
}

fn main() {

    /*
    thread::spawn(move || transmit_data(bridged_rx, ext_tx, outbound_iface_mac));
    thread::spawn(move || receive_data(bridged_tx));
    */
}
