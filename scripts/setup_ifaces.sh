#! /bin/bash
ip link add veth0 type veth peer name veth1
ip link set veth0 up
ip link set veth1 up
