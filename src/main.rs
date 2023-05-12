use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ip_string: &String = &args[1];

    println!("Pinging!");
    let ip: IpAddr = ip_string.parse().expect("Valid IP Address");
    for port in 1..=65535 {
        let conn: SocketAddr = SocketAddr::new(ip, port);
        match TcpStream::connect_timeout(&conn, Duration::from_millis(100)) {
            Ok(_) => println!("{:5} is open", port),
            Err(_) => {}
        }
    }
}
