use std::os::raw::{c_int, c_void};

const AF_INET: c_int = 1; //ipv4
const SOCK_RAW: c_int = 3; //for raw socket, which lets you build your own packets, including headers
const IPPROTO_ICMP: c_int = 1; //tells the kernel which protocol you’re using on top of IP — here it’s ICMP
const IPPROTO_IP: c_int = 0; //this mean the ip protocol itself
const IP_HDRINCL: c_int = 3; //This socket option tells the kernel whether you include the IP header yourself when sending packets

// FFI bindings for the tiny set of libc functions we need

unsafe extern "C" {

    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: u32,
    ) -> c_int;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: u32,
    ) -> isize;
    fn close(fd: c_int) -> c_int;

}

// This struct represents an IPv4 address, the same as the C type
#[repr(C)]
struct in_addr {
    s_addr: u32, // network byte order
}

// This is the IPv4-specific socket address structure, equivalent to C’s
#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

// This is the generic address structure used by syscalls like sendto()
#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [i8; 14],
}

pub fn create_sock() -> Result<i32, String> {
    unsafe {
        // create raw socket
        let fd = socket(AF_INET, SOCK_RAW, IPPROTO_ICMP);
        if fd < 0 {
            return Err("socket() failed (need root)".into());
        }

        Ok(fd)
    }
}

pub fn set_socket(fd: i32) -> Result<(), String> {
    unsafe {
        // set IP_HDRINCL = 1 so kernel uses our header
        let optval: i32 = 1;
        let ret = setsockopt(
            fd,
            IPPROTO_IP,
            IP_HDRINCL,
            &optval as *const _ as *const c_void,
            4 as u32,
        );
        if ret != 0 {
            return Err("setsockopt(IP_HDRINCL) failed".into());
        }
    }

    Ok(())
}
