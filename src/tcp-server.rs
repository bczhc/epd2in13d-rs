use std::io;
use std::io::{Read, Write};
/// ## Protocol
///
/// ### Request
///
/// \[ Directive | Image Buffer \]
///
/// - Directive: 16 bytes; a zero-terminated string
/// - Image Buffer: [`ffi::EPD_2IN13D_WIDTH`] / 8 * [`ffi::EPD_2IN13D_HEIGHT`] = 2756 (bytes)
///    See https://www.waveshare.net/wiki/2.13inch_e-Paper_HAT_(D)_Manual#%E7%BC%96%E7%A8%8B%E5%8E%9F%E7%90%86 for reference.
///
///
/// ### Response
///
/// `b"done\n"`
use std::net::{SocketAddr, TcpListener, TcpStream};

use clap::Parser;

use epd2in13d_rs::dev;
use epd2in13d_rs::epd2in13d::Epd2in13d;
use epd2in13d_rs::ffi::{EPD_2IN13D_HEIGHT, EPD_2IN13D_WIDTH};

#[derive(Debug, clap::Parser)]
struct Args {
    /// Port the server listens on
    port: u16,
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut directive_buffer = [0_u8; 16];
    stream.read_exact(&mut directive_buffer)?;
    let end = directive_buffer
        .iter()
        .position(|&x| x == 0x00)
        .unwrap_or(directive_buffer.len());
    let string_slice = &directive_buffer[..end];
    let directive = &*String::from_utf8_lossy(string_slice);

    println!("Directive: {directive}");

    println!("Performing screen update...");
    match directive {
        "update" | "updatePart" => {
            let image_buffer_len = EPD_2IN13D_WIDTH / 8 * EPD_2IN13D_HEIGHT;
            let mut image_buffer = vec![0_u8; image_buffer_len];
            stream.read_exact(&mut image_buffer)?;

            Epd2in13d.init();
            if directive == "updatePart" {
                Epd2in13d.display_part(&image_buffer);
            } else {
                Epd2in13d.display(&image_buffer);
            }
            Epd2in13d.sleep();
        }
        "clear" => {
            Epd2in13d.init();
            Epd2in13d.clear();
            Epd2in13d.sleep();
        }
        _ => {
            unimplemented!();
        }
    }
    println!("Done");

    stream.write_all(b"done\n")?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    dev::module_init()?;

    let socket_addr = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(socket_addr.parse::<SocketAddr>().unwrap())?;
    println!("Listening on {socket_addr}");
    loop {
        let (stream, from) = listener.accept()?;
        println!("Accepted: {from}");
        handle_connection(stream)?;
    }
}
