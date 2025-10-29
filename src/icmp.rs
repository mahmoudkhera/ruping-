use crate::net_utils::{rfc1071_checksum, struct_to_bytes};

pub enum KIND {
    ECHO,
    ECHOREPLY,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RawICMP {
    r#type: u8,
    code: u8,
    checksum: u16,
}

impl RawICMP {
    pub fn new(kind: KIND) -> Self {
        match kind {
            KIND::ECHO => Self {
                r#type: 8,
                code: 0,
                checksum: 0,
            },
            KIND::ECHOREPLY => Self {
                r#type: 0,
                code: 0,
                checksum: 0,
            },
        }
    }

    pub fn evaluate_icmp(&mut self, data: &[u8]) -> Vec<u8> {
        let mut packet = struct_to_bytes(self); // exact header bytes
        packet.extend_from_slice(data); // append payload

        set_icmp_checksum(&mut packet);

        packet
    }
}

//helper function

fn set_icmp_checksum(icmp_packet: &mut Vec<u8>) {
    let checksum = rfc1071_checksum(&icmp_packet);
    let checksum_bytes = checksum.to_be_bytes();

    icmp_packet[10] = checksum_bytes[0];
    icmp_packet[11] = checksum_bytes[1];
}
