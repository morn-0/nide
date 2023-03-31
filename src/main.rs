use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::Command,
};

fn main() {
    let mut args = env::args();
    let path = args.nth(1);

    if let Some(path) = path {
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8008") {
            let mut buf = [0u8; 4];
            if let Err(e) = stream.read_exact(&mut buf) {
                eprintln!("{e}");
            }
            let port = i32::from_be_bytes(buf);

            if let Err(e) = Command::new("nvim")
                .arg(path)
                .arg("--headless")
                .arg("--listen")
                .arg(format!("127.0.0.1:{}", port))
                .spawn()
            {
                eprintln!("{e}");
                return;
            }

            if let Err(e) = stream.write_all(&[1]) {
                eprintln!("{e}");
            }
        }
    } else {
        if let Ok(listener) = TcpListener::bind("127.0.0.1:8008") {
            while let Ok((mut stream, _)) = listener.accept() {
                if let Some(port) = portpicker::pick_unused_port() {
                    if let Err(e) = stream.write_all(&port.to_be_bytes()) {
                        eprintln!("{e}");
                    }

                    let mut buf = [0u8; 1];
                    if let Err(e) = stream.read_exact(&mut buf) {
                        eprintln!("{e}");
                    }

                    if buf[0] == 1 {
                        if let Err(e) = Command::new("neovide")
                            .arg("--remote-tcp")
                            .arg(format!("127.0.0.1:{}", port))
                            .spawn()
                        {
                            eprintln!("{e}");
                        }
                    }
                }
            }
        }
    }
}
