use std::process::exit;
use std::thread;
use std::{
    io::{self, prelude::*, BufReader, Write},
    net::TcpStream,
    str,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "args")]
struct ClientOpt {
    /// is hex string
    #[structopt(short = "h", long = "hex")]
    is_hex: bool,

    /// tcp url
    #[structopt(short)]
    url: String,
}

fn main() {
    let opt = ClientOpt::from_args();
    println!("{:?}", &opt);

    let mut stream = TcpStream::connect(&opt.url).expect("Failed to connect ");

    let recv_stream = stream.try_clone().expect("Failed to clone stream");

    let is_hex = opt.is_hex;

    let read_thr = thread::spawn(move || handle_recv(recv_stream, is_hex));

    let write_thr = thread::spawn(move || loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        let mut b1: Vec<u8> = Vec::new();

        let bytes: &[u8] = if is_hex {
            let b = input
                .split_ascii_whitespace()
                .map(|s| u8::from_str_radix(s, 16).expect("Failed to parse str to bytes "));

            println!("push in vec");
            b.for_each(|c| {
                print!("{:02x} ", &c);
                b1.push(c);
            });
            &b1[..]
        } else {
            input.as_bytes()
        };

        stream.write(bytes).ok();
        println!("\nSent");
    });

    read_thr.join().expect("join error");

    write_thr.join().expect("join error");
}

fn handle_recv(stream: TcpStream, is_hex: bool) {
    loop {
        let mut reader = BufReader::new(&stream);
        let mut buffer = [0u8; 1024];
        let size = reader
            .read(&mut buffer)
            .expect("Could not read into buffer");

        println!("recv: ({})", size);

        if size == 0 {
            exit(1);
        }

        if is_hex {
            buffer
                .iter()
                .take(size)
                .for_each(|item| print!("{:02x} ", item));

            println!();
        } else {
            println!(
                "{}",
                str::from_utf8(&buffer).expect("Failed to parse bytes to utf8")
            );
        }
    }
}
