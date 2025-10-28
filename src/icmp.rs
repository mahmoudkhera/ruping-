use crate::net_utils::{rfc1071_checksum, struct_to_bytes};

pub enum KIND {
    ECHO,
    ECHOREPLY,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RAWICMP {
    r#type: u8,
    code: u8,
    checksum: u16,
}

impl RAWICMP {
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
        
        let checksum = rfc1071_checksum(&packet);
        let checksum_bytes = checksum.to_be_bytes();
        packet[2] = checksum_bytes[0];
        packet[3] = checksum_bytes[1];

        packet
    }
}
