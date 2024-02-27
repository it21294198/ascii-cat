use std::io::{BufRead, Write};
use std::net::TcpListener;
use std::io::prelude::*;

use image::GenericImageView;

fn get_str_ascii(intent:u8)-> &'static str{
    let index = intent/32;
    let ascii = [" ",".",",","-","~","+","=","@"];
    return ascii[index as usize];
}

fn get_image(dir: &str,scale:u32){
    let img = image::open(dir).unwrap();
    // println!("{:?}",img.dimensions());
    let (width,height) = img.dimensions();
    for y in 0..height{
        for x in 0..width{
            if y % (scale*2) == 0 && x % scale == 0 {
                let pix = img.get_pixel(x, y);
                let mut intent = pix[0]/3 + pix[1]/3 + pix[2]/3;
                if pix[3]==0 {
                    intent = 0;     
                }
                print!("{}",get_str_ascii(intent));
            }
        }
        if y%(scale*2)==0{
            println!("");
        }
    }
}

use get_if_addrs::{get_if_addrs, IfAddr};

fn main() {
    let mut ip_address:String="".to_owned();
    if let Ok(interfaces) = get_if_addrs() {
        for interface in interfaces {
            if let IfAddr::V4(addr) = interface.addr {
                ip_address = addr.ip.to_string();
                // println!("  IPv4 Address: {}", addr.ip);
            }
        }
    } else {
        println!("Failed to get network interfaces");
    }

    get_image("cat.png", 10);
    let qr_code = format!("http://{}:3000/index.html",ip_address);
    let _ = qr2term::print_qr(qr_code);
    println!("http://{ip_address}:3000/index.html");
    
    // web server part
    let listener = TcpListener::bind("192.168.1.22:3000").unwrap();
    for mut stream in listener.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&mut stream);
        let mut l = String::new();
        rdr.read_line(&mut l).unwrap();
        match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut l = String::new();
                    rdr.read_line(&mut l).unwrap();
                    if l.trim().is_empty() {
                        break;
                    }
                }
                let mut p = std::path::PathBuf::new();
                p.push("src");
                p.push(resource.trim_start_matches("/"));

                let response = match std::fs::read_to_string(&p) {
                    Ok(content) => format!("HTTP/1.1 200 OK\r\n\r\n{}", content),
                    Err(_) => "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string(),
                };
                stream.write_all(response.as_bytes()).unwrap();
            }
            _ => todo!(),
        }
    }

    // simple web server part
    // let listener = std::net::TcpListener::bind("127.0.0.1:3000").unwrap();
    // for mut stream in listener.incoming().flatten(){
    //     let mut rdr = std::io::BufReader::new(&mut stream);
    //     loop{
    //         let mut l = String::new();
    //         rdr.read_line(&mut l).unwrap();
    //         if l.trim().is_empty(){ break;}
    //         print!("{l}");
    //     }
    //     stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello!").unwrap();
    // }
}
