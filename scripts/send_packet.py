from scapy.all import *

INTERFACE = "veth1"

dummy_packet = Ether()/IP(dst="192.168.203.129")/TCP(dport=80)/"GET /index.html HTTP/1.0\r\n\r\n"
sendp(dummy_packet, iface=INTERFACE)
