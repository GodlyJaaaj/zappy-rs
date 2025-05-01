mod footer;
mod navbar;
mod network;

use crate::footer::{Footer, FooterMessage};
use crate::navbar::{Navbar, NavbarMessage};
use crate::network::{GuiToServerMessage, NetworkInput, NetworkOutput, network_worker};
use env_logger::Env;
use iced::futures::channel::mpsc;
use iced::widget::{column, text};
use iced::window::Settings;
use iced::{Element, Length, Size, Subscription};
use log::{error, info, warn};
use std::default::Default;
use std::net::{Ipv4Addr, SocketAddrV4};

pub fn main() -> iced::Result {
    env_logger::Builder::from_env(Env::default().default_filter_or("gui=trace")).init();

    let mut windows_settings = Settings::default();
    windows_settings.min_size = Some(Size::new(600.0, 600.0));
    windows_settings.size = Size::new(800.0, 600.0);

    iced::application(
        "Zappy GUI made By Sébastien LUCAS",
        ZappyGui::update,
        ZappyGui::view,
    )
    .window(Settings::from(windows_settings))
    .centered()
    .antialiasing(true)
    .subscription(ZappyGui::start_network)
    .run()
}

#[derive(Debug, Clone)]
enum Message {
    Navbar(NavbarMessage),
    Footer(FooterMessage),
    Network(NetworkOutput),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tab {
    #[default]
    Map,
    Logs,
    Settings,
}
#[derive(Default)]
struct ZappyGui {
    navbar: Navbar,
    footer: Footer,
    //network worker channel should never be closed
    network: Option<mpsc::Sender<NetworkInput>>,
    //network channel to send commands directly to the server
    active_connection: Option<mpsc::Sender<GuiToServerMessage>>,
}

impl ZappyGui {
    fn update(&mut self, message: Message) {
        match message {
            Message::Navbar(navbar_message) => match navbar_message {
                NavbarMessage::Connect(ref ip, ref port) => {
                    let socket_addr = match ip.parse::<Ipv4Addr>() {
                        Ok(ip) => SocketAddrV4::new(ip, port.parse::<u16>().unwrap()),
                        Err(err) => {
                            error!("Invalid IP address {}", err);
                            return;
                        }
                    };
                    self.navbar.update(navbar_message.clone());
                    let _ = self
                        .network
                        .as_mut()
                        .unwrap()
                        .try_send(NetworkInput::Connect(socket_addr));
                }
                NavbarMessage::Disconnect => {
                    if let Some(network_sender) = &mut self.network {
                        let _ = network_sender.try_send(NetworkInput::Disconnect);
                    }
                    self.active_connection = None;
                    self.navbar.update(navbar_message);
                    self.footer.update(FooterMessage::ConnectionStatusChanged(
                        footer::ConnectionStatus::Disconnected,
                    ));
                }
                _ => {
                    self.navbar.update(navbar_message);
                }
            },
            Message::Footer(footer_message) => {
                self.footer.update(footer_message);
            }
            Message::Network(event) => match event {
                NetworkOutput::Ready(sender) => {
                    info!("Network is ready");
                    self.network = Some(sender);
                    self.footer.update(FooterMessage::ConnectionStatusChanged(
                        footer::ConnectionStatus::Disconnected,
                    ));
                    self.navbar
                        .set_connection_state(navbar::ConnectionState::Disconnected);
                }
                NetworkOutput::Connected(addr, cmd_sender) => {
                    info!("Network is connected to {}", addr);
                    self.active_connection = Some(cmd_sender);
                    self.footer.update(FooterMessage::ConnectionStatusChanged(
                        footer::ConnectionStatus::Connected(addr),
                    ));
                    self.navbar
                        .set_connection_state(navbar::ConnectionState::Connected);
                }
                NetworkOutput::Disconnected => {
                    warn!("Network is disconnected, connection closed");
                    self.active_connection = None;
                    self.footer.update(FooterMessage::ConnectionStatusChanged(
                        footer::ConnectionStatus::Disconnected,
                    ));
                    self.navbar
                        .set_connection_state(navbar::ConnectionState::Disconnected);
                }
                NetworkOutput::ConnectionFailed(addr, error) => {
                    error!("Network failed to connect to {}", addr);
                    self.footer.update(FooterMessage::ConnectionStatusChanged(
                        footer::ConnectionStatus::ConnectionFailed(error),
                    ));
                    self.navbar
                        .set_connection_state(navbar::ConnectionState::Disconnected);
                }
            },
        }
    }

    fn start_network(&self) -> Subscription<Message> {
        Subscription::run(network_worker).map(Message::Network)
    }

    fn view(&self) -> Element<Message> {
        let navbar = self.navbar.view().map(Message::Navbar);

        let content = match self.navbar.active_tab() {
            Tab::Settings => column![text("Information sur l'application")],
            Tab::Map => column![text("Information sur la map")],
            Tab::Logs => column![text("Information sur les logs")],
        };

        let footer = self.footer.view().map(Message::Footer);

        column![navbar, content.height(Length::Fill), footer].into()
    }
}
