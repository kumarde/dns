use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

struct RR_Header {
    name: String,
    type: u16,
    class: u16,
    ttl: u32,
    rdlength: u16 
}

struct CNAME {
    header: RR_Header,
    target: String     
}

struct HINFO {
    header: RR_Header,
    cpu: String,
    os: String    
}

struct MB{
    header: RR_Header,
    madname: String    
}

struct MD{
    header: RR_Header,
    madname: String    
}

struct MF {
    header: RR_Header,
    madname: String    
}

struct MG {
    header: RR_Header,
    mgname: String    
}

struct MINFO {
    header: RR_Header,
    rmailbx: String,
    emailbx: String    
} 

struct MR {
    header: RR_Header,
    newname: String    
}

struct MX {
    header: RR_Header,
    preference: i16,
    exchange: String 
}

struct NS {
    header: RR_Header,
    nsdname: String 
}

struct PTR {
    header: RR_Header,
    ptrdname: String    
}

struct SOA {
    header: RR_Header,
    mname: String,
    rname: String,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32
    minimum: u32
}

struct TXT {
    header: RR_Header,
    txtdata: Vec<String>; 
}

struct A {
    header: RR_Header,
    address: IpAddr 
}
