use std::net::{IpAddr, UdpSocket};

struct RR_Header {
    name: String,
    type: u16,
    class: u16,
    ttl: u32,
    rdlength: u16 
}

struct A {
    header: RR_Header,
    address: IpAddr    
}

struct MessageHeader{
    id: u16,
    bits: u16,
    qdcount, ancount, nscount, arcount: u16  
}

struct Question {
    name: String,
    Qtype: u16,
    Qclass: u16     
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let mut socket = UdpSocket::bind("127.0.0.1:45678").expect("couldn't bind to address");
    socket.send_to(&[0; 10], "8.8.8.8:53").expect("couldn't send data");
}

fn construct_message(domain: &String) {
    //   
}
