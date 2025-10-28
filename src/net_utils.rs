//Reference formula (RFC 1071)
// 1. Initialize sum = 0
// 2. Add each 16-bit word to sum
// 3. Fold carry bits: sum = (sum & 0xFFFF) + (sum >> 16)
// 4. One’s complement: checksum = ~sum

use std::{mem, slice};

/// Compute the 16-bit Internet checksum (RFC 1071)
pub fn rfc1071_checksum(data: &[u8]) -> u16 {
    // We start with a 32-bit accumulator (sum).
    // Even though the final checksum is 16 bits,
    // we use 32 bits to handle carry bits safely when we add many 16-bit numbers.
    let mut sum: u32 = 0;

    // process 16-bit chunks
    let mut chunks = data.chunks_exact(2);
    for chunk in &mut chunks {
        let word = u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
        sum = sum.wrapping_add(word);
    }

    // if there’s a leftover byte (odd length)
    if let Some(&byte) = chunks.remainder().first() {
        let word = (byte as u32) << 8; // pad with zero on the right
        sum = sum.wrapping_add(word);
    }

    // fold carries from high 16 bits into low 16 bits
    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    // one's complement and truncate to 16 bits
    !(sum as u16)
}

// Convert any plain struct into bytes
//# Safety
//- T must be POD (no references, no Vec, no String)
// - T must have stable layout (repr(C) or repr(packed))
pub fn struct_to_bytes<T>(s: &T) -> Vec<u8> {
    unsafe {
        let ptr = s as *const T as *const u8;
        let size = mem::size_of::<T>();
        slice::from_raw_parts(ptr, size).to_vec()
    }
}
