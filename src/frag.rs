/// Fragmentation Library

extern crate pnet;

use pnet::packet::{Packet, MutablePacket};

pub trait ProxyFragmenter {
    fn init(&self, packet: Box<Packet>, size: usize) -> Result<(), ()>;
    fn next(&self) -> Result<Option<Box<Packet>>, &str>;
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
    fn init(&self, packet: Box<Packet>, size: usize) -> Result<(), ()> {
        Err(())
    }

    fn next(&self) -> Result<Option<Box<Packet>>, &str> {
        Err("Unimplemented")
    }
}
