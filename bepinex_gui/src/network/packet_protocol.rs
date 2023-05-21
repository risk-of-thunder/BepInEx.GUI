use byteorder::{NativeEndian, ReadBytesExt};

use std::io::{Cursor, Read};

use std::mem::size_of;
use std::net::TcpStream;

use crate::bepinex_log::LogLevel;

pub(crate) fn read_packet_length(tcp_stream: &mut TcpStream) -> Result<usize, std::io::Error> {
    const HEADER_SIZE: usize = size_of::<u32>();

    let mut received_bytes = read_packet_internal(tcp_stream, HEADER_SIZE)?;

    let packet_length: usize =
        Cursor::new(&mut received_bytes).read_u32::<NativeEndian>()? as usize;

    Ok(packet_length)
}

pub(crate) fn read_packet_log_level(
    tcp_stream: &mut TcpStream,
) -> Result<LogLevel, std::io::Error> {
    unsafe {
        let mut received_bytes = read_packet_internal(tcp_stream, size_of::<i32>())?;

        let log_level: LogLevel = std::mem::transmute::<i32, LogLevel>(
            Cursor::new(&mut received_bytes).read_i32::<NativeEndian>()?,
        );

        Ok(log_level)
    }
}

pub(crate) fn read_packet(
    tcp_stream: &mut TcpStream,
    size_to_read: usize,
) -> Result<Vec<u8>, std::io::Error> {
    let packet_bytes = read_packet_internal(tcp_stream, size_to_read)?;

    Ok(packet_bytes)
}

fn read_packet_internal(
    tcp_stream: &mut TcpStream,
    size_to_read: usize,
) -> Result<Vec<u8>, std::io::Error> {
    const BUFFER_SIZE: usize = 4096;

    let mut packet_bytes: Vec<u8> = vec![];
    let mut read_stream_buffer = vec![0u8; size_to_read];
    let mut remaining_size_to_read = size_to_read;

    loop {
        read_stream_buffer.clear();
        read_stream_buffer.resize(
            if remaining_size_to_read > BUFFER_SIZE {
                BUFFER_SIZE
            } else {
                remaining_size_to_read
            },
            0,
        );

        match tcp_stream.read(&mut read_stream_buffer) {
            Ok(bytes_read) => {
                packet_bytes.extend_from_slice(&read_stream_buffer[..bytes_read]);

                remaining_size_to_read -= bytes_read;

                if remaining_size_to_read <= 0 {
                    break;
                }
            }
            Err(err) => return Err(err),
        }
    }

    Ok(packet_bytes)
}

pub(crate) fn packet_bytes_to_utf8_string(packet_bytes: &Vec<u8>) -> String {
    unsafe { std::str::from_utf8_unchecked(&packet_bytes).to_string() }
}
