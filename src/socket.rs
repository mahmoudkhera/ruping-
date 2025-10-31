use std::{
    io, mem,
    net::Ipv4Addr,
    os::raw::{c_int, c_void},
};

const AF_INET: c_int = 2; //ipv4
const SOCK_RAW: c_int = 3; //for raw socket, which lets you build your own packets, including headers
const IPPROTO_RAW: c_int = 255; //tells the kernel which protocol you’re using on top of IP — here it’s ICMP

// FFI bindings for the tiny set of libc functions we need

unsafe extern "C" {

    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;

    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: u32,
    ) -> isize;

}

pub struct in_addr {
    pub s_addr: u32,
}


#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [i8; 14],
}

pub fn create_sock() -> Result<i32, String> {
    unsafe {
        // create raw socket
        let fd = socket(AF_INET, SOCK_RAW, IPPROTO_RAW);
        if fd < 0 {
            let err = io::Error::last_os_error();
            return Err(format!("socket() failed {}", err).into());
        }

        Ok(fd)
    }
}

pub fn send_ipv4(fd: i32, pkkt: &[u8], dst: Ipv4Addr) -> Result<(), String> {
    unsafe {
        // build sockaddr_in for destination
        let mut addr: sockaddr_in = mem::zeroed();
        addr.sin_family = AF_INET as u16;
        addr.sin_port = 0;
        // in_addr expects network order (big-endian u32)
        addr.sin_addr.s_addr = u32::from_ne_bytes(dst.octets());

        // call sendto
        let ret = sendto(
            fd,
            pkkt.as_ptr() as *const c_void,
            pkkt.len(),
            0,
            &addr as *const sockaddr_in as *const sockaddr,
            mem::size_of::<sockaddr_in>() as u32,
        );
        if ret < 0 {
            let err = io::Error::last_os_error();
            return Err(format!("socket() failed {}", err).into());
        } else {
            Ok(())
        }
    }
}
