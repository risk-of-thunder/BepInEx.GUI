use core::time;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpStream;

use std::io;

use std::thread;

use std::sync::mpsc::Sender;

use crate::bepinex_log::BepInExLog;
use crate::bepinex_log::LogLevel;
use crate::packet_protocol;

#[derive(Clone)]
pub struct LogReceiverThread {
    log_socket_port_receiver: u16,
    channel_sender: Sender<BepInExLog>,
}

impl LogReceiverThread {
    pub fn new(
        log_socket_port_receiver: u16,
        channel_sender: Sender<BepInExLog>,
    ) -> LogReceiverThread {
        LogReceiverThread {
            log_socket_port_receiver: log_socket_port_receiver,
            channel_sender: channel_sender,
        }
    }

    pub fn start_thread_loop(&self) {
        let server_address = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            self.log_socket_port_receiver,
        );
        let inst = self.clone();
        thread::spawn(move || -> io::Result<()> {
            loop {
                match TcpStream::connect(server_address) {
                    Ok(mut tcp_stream) => loop {
                        match packet_protocol::read_packet_length(&mut tcp_stream) {
                            Ok(packet_length) => {
                                match packet_protocol::read_packet_log_level(&mut tcp_stream) {
                                    Ok(log_level) => {
                                        match packet_protocol::read_packet(
                                            &mut tcp_stream,
                                            packet_length,
                                        ) {
                                            Ok(packet_bytes) => {
                                                inst.send_bepinex_log_packet_to_channel(
                                                    log_level,
                                                    &packet_bytes,
                                                );
                                            }
                                            Err(err) => {
                                                tracing::error!(
                                                    "Error reading packet: {}\nDisconnecting socket",
                                                    err
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        tracing::error!(
                                            "Error reading packet log level: {}\nDisconnecting socket",
                                            err
                                        );
                                        break;
                                    }
                                }
                            }
                            Err(err) => {
                                tracing::error!(
                                    "Error reading packet length: {}\nDisconnecting socket",
                                    err
                                );
                                break;
                            }
                        }
                    },
                    Err(err) => tracing::error!("Failed connecting: {}", err),
                }

                const DELAY_IN_MS_BETWEEN_CONNECTION_TRY: u64 = 2000;
                thread::sleep(time::Duration::from_millis(
                    DELAY_IN_MS_BETWEEN_CONNECTION_TRY,
                ));
            }
        });
    }

    fn send_bepinex_log_packet_to_channel(
        &self,
        log_level: LogLevel,
        string_packet_bytes: &Vec<u8>,
    ) {
        let log_string = packet_protocol::packet_bytes_to_utf8_string(&string_packet_bytes);
        let bepinex_log = BepInExLog::new(log_level, log_string);
        if let Err(err) = &self.channel_sender.send(bepinex_log) {
            tracing::error!("error while sending utf8 string to channel: {}", err);
        }
    }
}
