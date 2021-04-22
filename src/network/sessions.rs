use std::{
    collections::HashMap,
    io::{Read, Write},
    str::from_utf8,
};

extern crate mio;

// TODO: Benchmark direct kqueue calls vs mio at some point

// shamelessly stolen from: https://github.com/tokio-rs/mio/blob/1667a7027382bd43470bc43e5982531a2e14b7ba/examples/tcp_server.rs

fn next(current: &mut mio::Token) -> mio::Token {
    let next = current.0;
    current.0 += 1;
    mio::Token(next)
}

const IN: mio::Token = mio::Token(0);
const OUT: mio::Token = mio::Token(1);
const ERR: mio::Token = mio::Token(2);

const SERVER: mio::Token = mio::Token(3);

enum IntConnection<'a> {
    TcpStream(mio::net::TcpStream),
    SourceFd(mio::unix::SourceFd<'a>),
}

pub fn loop_sessions(host: &str, port: u16) -> std::io::Result<()> {
    let mut poll = mio::Poll::new()?;
    let mut events = mio::Events::with_capacity(128);

    let addr = [host, port.to_string().as_str()].join(":").parse().unwrap();
    let mut server = mio::net::TcpListener::bind(addr)?;

    let stdin = 0;
    let mut stdin_fd = mio::unix::SourceFd(&stdin);
    poll.registry()
        .register(&mut stdin_fd, IN, mio::Interest::READABLE)?;
    poll.registry()
        .register(&mut server, SERVER, mio::Interest::READABLE)?;

    let mut connections = HashMap::new();
    let mut unique_token = mio::Token(SERVER.0 + 1);

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => loop {
                    let (mut connection, address) = match server.accept() {
                        Ok((connection, address)) => (connection, address),
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            break;
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    println!("Accepted connection from: {}", address);

                    let token = next(&mut unique_token);
                    poll.registry().register(
                        &mut connection,
                        token,
                        mio::Interest::READABLE.add(mio::Interest::WRITABLE),
                    )?;

                    connections.insert(token, connection);
                },
                IN => {
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line)?;
                    println!("{}", line);

                    poll.registry()
                        .reregister(&mut stdin_fd, IN, mio::Interest::READABLE)?
                }
                token => {
                    let done = if let Some(connection) = connections.get_mut(&token) {
                        handle_connection_event(poll.registry(), connection, event)?
                    } else {
                        // TODO: WTF is that stuff, investigate
                        false
                    };
                    if done {
                        connections.remove(&token);
                    }
                }
            }
        }
    }
}

const DATA: &[u8] = b"Hello world!\n";

fn handle_connection_event(
    registry: &mio::Registry,
    connection: &mut mio::net::TcpStream,
    event: &mio::event::Event,
) -> std::io::Result<bool> {
    if event.is_writable() {
        match connection.write(DATA) {
            Ok(n) if n < DATA.len() => return Err(std::io::ErrorKind::WriteZero.into()),
            Ok(_) => registry.reregister(connection, event.token(), mio::Interest::READABLE)?,
            Err(ref err) if would_block(err) => {}
            Err(ref err) if interrupted(err) => {
                return handle_connection_event(registry, connection, event)
            }
            Err(err) => return Err(err),
        }
    }

    if event.is_readable() {
        let mut connection_closed = false;
        let mut received_data = vec![0; 4096];
        let mut bytes_read = 0;
        loop {
            match connection.read(&mut received_data[bytes_read..]) {
                Ok(0) => {
                    connection_closed = true;
                    break;
                }
                Ok(n) => {
                    bytes_read += n;
                    if bytes_read == received_data.len() {
                        received_data.resize(received_data.len() + 1024, 0);
                    }
                }
                Err(ref err) if would_block(err) => break,
                Err(ref err) if interrupted(err) => continue,
                Err(err) => return Err(err),
            }
        }

        if bytes_read != 0 {
            let received_data = &received_data[..bytes_read];
            if let Ok(str_buf) = from_utf8(received_data) {
                println!("Received data: {}", str_buf.trim_end());
            } else {
                println!("Received (none UTF-8) data: {:?}", received_data);
            }
        }

        if connection_closed {
            println!("Connection closed");
            return Ok(true);
        }
    }

    Ok(false)
}

fn would_block(err: &std::io::Error) -> bool {
    err.kind() == std::io::ErrorKind::WouldBlock
}

fn interrupted(err: &std::io::Error) -> bool {
    err.kind() == std::io::ErrorKind::Interrupted
}
