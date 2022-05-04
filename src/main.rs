use std::env;
use std::io;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::process::Stdio;

fn main() -> io::Result<()> {
    let addr = env::args().nth(1).unwrap();
    let mut server = Server::bind(&addr)?;

    server.listening()?;

    Ok(())
}

struct Server {
    listener: TcpListener,
    buf: String,
}

impl Server {
    fn bind(addr: &str) -> io::Result<Self> {
        let l = TcpListener::bind(addr)?;
        println!("Binding to {}", addr);

        Ok(Self {
            listener: l,
            buf: String::new(),
        })
    }

    fn listening(&mut self) -> io::Result<()> {
        println!("Server Listening...");

        #[allow(clippy::unused_io_amount)]
        for con in self.listener.incoming() {
            let mut con = con?;
            self.buf.clear();

            match con.read_to_string(&mut self.buf) {
                Ok(_) => {
                    println!("{}\n", self.buf);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }

        Ok(())
    }
}
