#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;

use std::io::prelude::*;
use mio::*;
use mio::tcp::TcpListener;

const SERVER: Token = Token(0);

fn main() {
    env_logger::init().unwrap();

    info!("Opening port 3000");
    let addr = "0.0.0.0:3000".parse().unwrap();
    let server = TcpListener::bind(&addr).unwrap();
    let poll = Poll::new().unwrap();
    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

    let mut events = Events::with_capacity(1024);
    loop {
        poll.poll(&mut events, None).unwrap();
        for event in events.iter() {
            match event.token() {
                SERVER => {
                    info!("Accepting connection on port 3000");
                    let (mut stream, peer_addr) = server.accept().unwrap();
                    debug!("Connection from {:?}", peer_addr);
                    stream.write_fmt(format_args!("Hello, World!")).unwrap();
                }
                _ => {
                    unreachable!();
                }
           }
        }
    }
}
