extern crate shout;

use std::io::{self, Read};

fn main() {
    let conn = shout::ShoutConnBuilder::new()
        .host(String::from("server"))
        .port(8000)
        .user(String::from("source"))
        .password(String::from("pw"))
        .mount(String::from("/test.ogg"))
        .protocol(shout::ShoutProtocol::HTTP)
        .format(shout::ShoutFormat::Ogg)
        .build().unwrap();

    println!("Connected to server");
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer).unwrap();
    let step = 4096;
    let mut pos = 0;
    loop {
        if pos + step < buffer.len() {
            conn.send(buffer[pos..(pos+step)].to_vec());
            pos += step;
            conn.sync();
        } else {
            conn.send(buffer[pos..(pos+(buffer.len() - pos))].to_vec());
            println!("Finished!");
            break;
        }
    }
}
