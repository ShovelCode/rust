use std::net::UdpSocket;
use std::io;


fn send_packets() -> io::Result<()> {
    // Bind to a local address.
    let socket = UdpSocket::bind("127.0.0.1:0")?;

    // Specify the remote address to which we will send data.
    let remote_address = "127.0.0.1:8080";

    for i in 0..10 {
        // Create a byte array containing the packet data.
        let data = format!("Packet number {}", i).into_bytes();

        // Send the packet.
        socket.send_to(&data, remote_address)?;
    }

    Ok(())
}

fn main() {
    match send_packets() {
        Ok(_) => println!("Successfully sent packets!"),
        Err(e) => eprintln!("Failed to send packets: {}", e),
    }
}
