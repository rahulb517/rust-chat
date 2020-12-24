use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;


const LOCAL: &str = "127.0.0.1:6000";
const MESSAGE_SIZE: usize = 64;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buf = vec![0; MESSAGE_SIZE];
        match client.read_exact(&mut buf) {
            Ok(_) => {
                let message = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                //println!("message recv {:?}", message);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }

        match rx.try_recv() {
            Ok(message) => {
                let mut buf = message.clone().into_bytes();
                buf.resize(MESSAGE_SIZE, 0);
                client.write_all(&buf).expect("writing to socket failed");
                println!("message sent {:?}", message);
            }, 
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a Message:");
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("reading from stdin failed");
        let message = buf.trim().to_string();
        if message == ":quit" || tx.send(message).is_err() {break}
    }
    println!("Good Talk!");

}