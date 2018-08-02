/// Fragmentation Library

extern crate pnet;

use pnet::packet::{Packet, MutablePacket};

pub trait ProxyFragmenter {
    fn init(&mut self, packet: Box<Packet>, size: usize) -> Result<(), &str>;
    fn next(&mut self) -> Result<Option<Box<Packet>>, &str>;
}

// Currently just a serial implementation. Later, will add support for a list
// of packets
pub struct ProxyFragStruct {
    packet: Box<Packet>,
    offset: usize,
}

impl ProxyFragStruct {
}

impl ProxyFragmenter for ProxyFragStruct {
    fn init(&mut self, packet: Box<Packet>, size: usize) -> Result<(), &str> {
        Err("Unimplemented")
        //self.packet = packet;
    }

    fn next(&mut self) -> Result<Option<Box<Packet>>, &str> {
        Err("Unimplemented")
    }
}
