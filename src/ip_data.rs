use std::{mem, net::Ipv4Addr, ptr};

use crate::net_utils::{rfc1071_checksum, struct_to_bytes};

#[derive(Debug, Clone, Copy)]
#[repr(u8)] // store enum as a single byte
pub enum IpProtocol {
    ICMP = 1,
    // add some protocols later
}

impl IpProtocol {
    /// Return the numeric protocol value.
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RawIpv4 {
    pub version_ihl: u8, // 4 bits version + 4 bits header length
    pub dscp_ecn: u8,    // 6 bits DSCP + 2 bits ECN
    pub total_length: u16,
    pub identification: u16,
    pub flags_fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8, // from IpProtocol enum
    pub header_checksum: u16,
    pub src_addr: [u8; 4],
    pub dst_addr: [u8; 4],
}

impl RawIpv4 {
    pub fn new(identification: u16, ttl: u8, src: Ipv4Addr, dst: Ipv4Addr) -> Self {
        let version = 4;
        let ihl = 5; // 5 × 32-bit words = 20 bytes (no options)
        let version_ihl = (version << 4) | ihl;

        RawIpv4 {
            version_ihl,
            dscp_ecn: 0,
            total_length: 0,
            identification: identification.to_be(),
            flags_fragment_offset: 0,
            ttl,
            protocol: 0,
            header_checksum: 0,
            src_addr: src.octets(),
            dst_addr: dst.octets(),
        }
    }

    pub fn evaluate_ipv4(&mut self, ip_protocol: IpProtocol, data: &[u8]) -> Vec<u8> {
        self.protocol = ip_protocol.to_u8();

        let total_len = (mem::size_of::<Self>() + data.len()) as u16;
        self.total_length = total_len.to_be();

        let mut packet = struct_to_bytes(self);
        set_ipv4_checksum(&mut packet);
        packet.extend_from_slice(data);

        packet
    }

    pub fn read_ip_header(buf: &[u8]) -> (RawIpv4, usize) {
        assert!(buf.len() >= std::mem::size_of::<RawIpv4>());

        let header = unsafe { ptr::read_unaligned(buf.as_ptr() as *const RawIpv4) };

        let ihl = (header.version_ihl & 0x0F) as usize * 4;

        let checksum_prime = rfc1071_checksum(&buf[0..ihl]);

        if checksum_prime != 0 {
            eprintln!("invalid ip packet")
        }

        (header, ihl)
    }

    pub fn get_src(&self) -> Ipv4Addr {
        Ipv4Addr::new(
            self.src_addr[0],
            self.src_addr[1],
            self.src_addr[2],
            self.src_addr[3],
        )
    }

    pub fn get_dest(&self) -> Ipv4Addr {
        Ipv4Addr::new(
            self.dst_addr[0],
            self.dst_addr[1],
            self.dst_addr[2],
            self.dst_addr[3],
        )
    }

    pub fn get_length(&self) -> u16 {
        self.total_length.to_be()
    }

    pub fn get_ttl(&self) -> u8 {
        self.ttl.to_le()
    }
}

//helper function
fn set_ipv4_checksum(ipv4_packet: &mut Vec<u8>) {
    let checksum = rfc1071_checksum(&ipv4_packet);
    let checksum_bytes = checksum.to_be_bytes();

    ipv4_packet[10] = checksum_bytes[0];
    ipv4_packet[11] = checksum_bytes[1];
}
