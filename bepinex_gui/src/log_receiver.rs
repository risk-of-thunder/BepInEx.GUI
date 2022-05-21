use std::net::TcpStream;

use std::io;

use std::thread;

use std::sync::mpsc::Sender;

use crate::bepinex_log::BepInExLog;
use crate::bepinex_log::LogLevel;
use crate::packet_protocol;

#[derive(Clone)]
pub struct LogReceiver {
    channel_sender: Sender<BepInExLog>,
}

impl LogReceiver {
    pub fn new(channel_sender: Sender<BepInExLog>) -> LogReceiver {
        LogReceiver {
            channel_sender: channel_sender,
        }
    }

    pub fn log_receiver_thread_loop(&self) {
        let inst = self.clone();
        thread::spawn(move || -> io::Result<()> {
            loop {
                match TcpStream::connect("127.0.0.1:27090") {
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
                                                eprintln!(
                                            "Error reading packet {}\n Disconnecting socket",
                                            err
                                        );
                                                break;
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        eprintln!(
                                            "Error reading packet log level {}\n Disconnecting socket",
                                            err
                                        );
                                        break;
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!(
                                    "Error reading packet length {}\n Disconnecting socket",
                                    err
                                );
                                break;
                            }
                        }
                    },
                    Err(err) => eprintln!("Failed connecting {}", err),
                }
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
            eprintln!("error while sending utf8 string to channel : {}", err);
        }
    }
}
