use pcap;
use std::collections::BTreeMap;
use chrono::prelude::*;
use std::{fs::OpenOptions, io::Write, mem, slice};

// const DLT_IEEE802_11_RADIO: i32 = 127;
const SNAPLEN: i32 = 4096;


fn main() {
    println!("Hello, world!");
    // parse_pcap();

    let mut capture = pcap::Capture::from_device(pcap::Device::lookup().unwrap())
        .unwrap()
        .timeout(1)
        .rfmon(true)
        .snaplen(SNAPLEN)
        .open()
        .unwrap();

    // capture.set_datalink(pcap::Linktype(DLT_IEEE802_11_RADIO))
    //     .unwrap();
    match capture.filter("tcp port 80",true){
        Ok(_) => {}
        Err(e) => {
           panic!("unhandled error:{:?}",e);
        }
    }

    let mut temp = OpenOptions::new()
        .create(true)
        .append(true)
        .open("port80.cap")
        .unwrap();

    loop {
        match capture.next() {
            Ok(packet) => {
                unsafe {
                    temp.write_all(any_as_u8_slice(&(packet.header.ts.tv_sec as u32))).unwrap();
                    temp.write_all(any_as_u8_slice(&(packet.header.ts.tv_usec as u32))).unwrap();
                    temp.write_all(any_as_u8_slice(&packet.header.caplen)).unwrap();
                    temp.write_all(any_as_u8_slice(&packet.header.len)).unwrap();
                }
                temp.write_all(&packet.data).unwrap();
                println!("len{} data{:?}",packet.header.len,packet.data);
            }
            Err(pcap::Error::TimeoutExpired) => continue,
            Err(e) => {
                panic!("unhandled error: {:?}", e);
            }
        }
    }

    // while  let _data=libpcap::next_ex(&mut packet){
    //     println!("packet head len:{:?}",packet.head.len);
    //     print!("{:?}",packet);
    // }
    // libpcap::close(&mut packet);
}
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    slice::from_raw_parts((p as *const T) as *const u8, mem::size_of::<T>())
}

fn parse_pcap(){
    let mut capture = pcap::Capture::from_file("port80.cap").unwrap();
    let mut distribution = BTreeMap::new();
    while let Ok(pkt) = capture.next() {
        let t = pkt.header.ts.tv_sec;
        let by_hour = t / 3600 * 3600;
        let dt = Local.timestamp(by_hour, 0);
        *distribution.entry(dt).or_insert(0) += 1;
    }

    for (t, c) in distribution {
        println!("{}: {:8}", t.format("%Y-%m-%d %H:%M"), c);
    }
}
