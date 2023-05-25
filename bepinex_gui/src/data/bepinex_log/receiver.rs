use core::time;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpStream;

use std::io;

use std::thread;

use crossbeam_channel::Sender;

use crate::backend::network::packet_protocol;
use crate::data::bepinex_mod::BepInExMod;

use super::BepInExLogEntry;
use super::LogLevel;

#[derive(Clone)]
pub struct LogReceiver {
    log_socket_port_receiver: u16,
    log_sender: Sender<BepInExLogEntry>,
    mod_sender: Sender<BepInExMod>,
}

impl LogReceiver {
    pub fn new(
        log_socket_port_receiver: u16,
        log_sender: Sender<BepInExLogEntry>,
        mod_sender: Sender<BepInExMod>,
    ) -> LogReceiver {
        LogReceiver {
            log_socket_port_receiver,
            log_sender,
            mod_sender,
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
                                                inst.make_log_entry_from_packet_data(
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

    fn make_log_entry_from_packet_data(&self, log_level: LogLevel, string_packet_bytes: &Vec<u8>) {
        let log_string = packet_protocol::packet_bytes_to_utf8_string(&string_packet_bytes);

        let log = BepInExLogEntry::new(log_level, log_string);

        if log.data().contains("Loading [") {
            let split: Vec<&str> = log.data().split('[').collect();
            let mod_info_text = split[2];
            let mod_version_start_index_ = mod_info_text.rfind(' ');
            if let Some(mod_version_start_index) = mod_version_start_index_ {
                let mod_name = &mod_info_text[0..mod_version_start_index];
                let mod_version =
                    &mod_info_text[mod_version_start_index + 1..mod_info_text.len() - 1];

                self.mod_sender
                    .send(BepInExMod::new(mod_name, mod_version))
                    .unwrap();
            }
        }

        self.log_sender.send(log).unwrap();
    }
}
