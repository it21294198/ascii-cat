use std::io::{BufRead, Write};

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
fn main() {
    get_image("cat.png", 10);

    let _ = qr2term::print_qr("http://127.0.0.1:3000/");
    
    let listener = std::net::TcpListener::bind("127.0.0.1:3000").unwrap();
    for mut stream in listener.incoming().flatten(){
        let mut rdr = std::io::BufReader::new(&mut stream);
        loop{
            let mut l = String::new();
            rdr.read_line(&mut l).unwrap();
            if l.trim().is_empty(){ break;}
            print!("{l}");
        }
        stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello!").unwrap();
    }


}
