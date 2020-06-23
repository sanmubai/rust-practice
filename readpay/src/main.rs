use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

fn main() {
    // println!("read begin");

    let f=File::open("./src/13_134820.log").unwrap();
    let buf=BufReader::new(f);
    let mut l_all =Vec::new();
    for line in buf.lines(){
        // println!("1 begin");
        let line_data=line.unwrap();
        let re = Regex::new(r"buyBundle:role (\d+) (ios_pay|google_pay|cafeBazaar_pay) (\d+) begin to call payItem").unwrap();
        for cap in re.captures_iter(&line_data) {
            // println!("rid: {} type: {} tid: {}",
            //          cap.at(1).unwrap_or(""), cap.at(2).unwrap_or(""),
            //          cap.at(3).unwrap_or(""));
            let mut v=Vec::new();
            let rid=cap.at(1).unwrap_or("");
            let tid=cap.at(3).unwrap_or("");
            let buytype=cap.at(2).unwrap_or("");
            v.push(rid.to_string());
            v.push(tid.to_string());
            v.push(buytype.to_string());
            l_all.push(v);
            break;
        }
        let re2=Regex::new(r"(ios_pay|google_pay|cafeBazaar_pay) successful").unwrap();
        if re2.is_match(&line_data){
            let len=l_all.len();
            if len<=0{
                continue;
            }
            l_all[len-1].push(String::from("ok"));
        }

        let re3=Regex::new(r"(pay failed!2 cancel|onPurchasesUpdated: user cancelled)").unwrap();
        if re3.is_match(&line_data){
            let len=l_all.len();
            if len<=0{
                continue;
            }
            l_all[len-1].push(String::from("cancel"));
        }

        let re4=Regex::new(r"pay failed!7 Item is already owned").unwrap();
        if re4.is_match(&line_data){
            let len=l_all.len();
            if len<=0{
                continue;
            }
            l_all[len-1].push(String::from("pay failed!7 Item is already owned"));
        }
        // println!("1 end");
    }
    print_line(l_all);
    // println!("read end");
}

fn print_line(v:Vec<Vec<String>>){
    for line in v{
        println!("{:?}",line);
    }
}
