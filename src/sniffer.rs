use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::Sender;

const MAX_PORT: u16 = 65535;

#[derive(Debug)]
pub struct Arguments {
    pub ipaddr: IpAddr,
    pub threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            Ok(Arguments { ipaddr, threads: 4 })
        } else {
            let flag = args[1].clone();

            if flag.contains("-h") || (flag.contains("--help") && args.len() == 2) {
                print_help();
                //eprintln!(
                //    "Usage: -j to select how many threads you want
                //    \r\n    -h or --help to show this help message."
                //);

                Err("help")
            } else if flag.contains("-h") || flag.contains("--help") {
                Err("too many arguments")
            } else if flag.contains("-j") {
                if args.len() <= 2 {
                    print_help();
                    return Err("not enough arguments");
                }

                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(num_thread) => num_thread,
                    Err(_) => return Err("failed to parse thread number"),
                };

                Ok(Arguments { threads, ipaddr })
            } else {
                Err("invalid syntax")
            }
        }
    }
}

pub fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        if TcpStream::connect((addr, port)).is_ok() {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }

        if (MAX_PORT - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn print_help() {
    eprintln!(
        "Usage: -j to select how many threads you want 
         \r\n\t-h or --help to show this help message.\n"
    );
}
