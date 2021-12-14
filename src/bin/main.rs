use ip_sniffer::{scan, Arguments};
use std::sync::mpsc::channel;
use std::{env, process, thread};

fn main() {
    // 1. Parse arguments
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} program parsing arguments -> {}", program, err);
            process::exit(1);
        }
    });

    // 2. Create multithreads to sacn ports
    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    // 3. Receive message and display to terminal
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!();
    out.sort_unstable();
    for v in out {
        println!("{} is open", v);
    }
}
