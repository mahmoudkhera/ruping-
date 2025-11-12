use std::ptr;

use crate::net_utils::{rfc1071_checksum, struct_to_bytes};

#[derive(Debug)]
#[repr(u8)] // store enum as a single byte
pub enum Kind {
    Echo,
    EchoReply,
    Other,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RawICMP {
    r#type: u8,
    code: u8,
    checksum: u16,
}

impl RawICMP {
    pub fn new(kind: Kind) -> Self {
        let mut typ = 0;
        let mut code = 0;
        match kind {
            Kind::Echo => {
                typ = 8;
                code = 0;
            }
            Kind::EchoReply => {
                typ = 0;
                code = 0;
            }
            Kind::Other => (),
        }

        Self {
            r#type: typ,
            code: code,
            checksum: 0,
        }
    }

    pub fn evaluate_icmp(&mut self, data: &[u8]) -> Vec<u8> {
        let mut packet = struct_to_bytes(self); // exact header bytes
        packet.extend_from_slice(data); // append payload

        set_icmp_checksum(&mut packet);

        packet
    }

    pub fn from_buf(buf: &mut [u8]) -> RawICMP {
        assert!(buf.len() > 8);

        let icmp_header = unsafe { ptr::read_unaligned(buf.as_ptr() as *const RawICMP) };
        let checksum_prime = rfc1071_checksum(&buf);

        println!("icmp check sum{:02x}", checksum_prime);

        icmp_header
    }

    pub fn get_kind(&self) -> Kind {
        let typ = self.r#type;
        let code = self.code;
        match (typ, code) {
            (8, 0) => Kind::Echo,
            (0, 0) => Kind::EchoReply,
            (_, _) => Kind::Other,
        }
    }
}

//helper function

fn set_icmp_checksum(icmp_packet: &mut Vec<u8>) {
    let checksum = rfc1071_checksum(&icmp_packet);
    let checksum_bytes = checksum.to_be_bytes();

    icmp_packet[2] = checksum_bytes[0];
    icmp_packet[3] = checksum_bytes[1];
}
