mod footer;
mod game;
mod navbar;
mod network;
mod views;

use crate::footer::{Footer, FooterMessage};
use crate::navbar::{ConnectionState, Navbar, NavbarMessage};
use crate::network::{
    GuiToServerMessage, NetworkInput, NetworkOutput, ServerMessage, network_worker,
};
use env_logger::Env;
use iced::futures::channel::mpsc;
use iced::widget::container::bordered_box;
use iced::widget::{column, container, text};
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
    .default_font(iced::Font::MONOSPACE)
    .subscription(ZappyGui::subscription)
    .run()
}

#[derive(Debug, Clone)]
enum Message {
    Navbar(NavbarMessage),
    Footer(FooterMessage),
    Network(NetworkOutput),
    Map(views::MapMessage),
    Settings(views::SettingsMessage),
    Logs(views::LogsMessage),
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
    // Vues
    map_view: views::MapView,
    settings_view: views::SettingsView,
    logs_view: views::LogsView,
    // network worker channel should never be closed
    network: Option<mpsc::Sender<NetworkInput>>,
    // network channel to send commands directly to the server
    active_connection: Option<mpsc::Sender<GuiToServerMessage>>,

    game_state: game::GameState,
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
                        .set_connection_state(ConnectionState::Disconnected);
                }
                NetworkOutput::Connected(addr, cmd_sender) => {
                    info!("Network is connected to {}", addr);
                    self.active_connection = Some(cmd_sender);
                    self.footer.update(FooterMessage::ConnectionStatusChanged(
                        footer::ConnectionStatus::Connected(addr),
                    ));
                    self.navbar.set_connection_state(ConnectionState::Connected);
                }
                NetworkOutput::Disconnected => {
                    warn!("Network is disconnected, connection closed");
                    self.active_connection = None;
                    self.game_state = game::GameState::default();
                    self.map_view = views::MapView::default();
                    self.footer.update(FooterMessage::ConnectionStatusChanged(
                        footer::ConnectionStatus::Disconnected,
                    ));
                    self.navbar
                        .set_connection_state(ConnectionState::Disconnected);
                }
                NetworkOutput::ConnectionFailed(addr, error) => {
                    error!("Network failed to connect to {}", addr);
                    self.footer.update(FooterMessage::ConnectionStatusChanged(
                        footer::ConnectionStatus::ConnectionFailed(error),
                    ));
                    self.navbar
                        .set_connection_state(ConnectionState::Disconnected);
                }
                NetworkOutput::ServerMessage(server_msg) => {
                    match server_msg {
                        ServerMessage::MapSize {
                            width: _width,
                            height: _height,
                        } => {
                            self.game_state.update_map_size(_width, _height);
                        }
                        ServerMessage::TeamName { name } => self.game_state.add_team(name),
                        ServerMessage::Other(_) => {
                            // Handle other messages if needed
                        }
                    }
                }
            },
            Message::Map(map_message) => {
                self.map_view.update(map_message);
            }
            Message::Settings(settings_message) => {
                self.settings_view.update(settings_message);
            }
            Message::Logs(logs_message) => {
                self.logs_view.update(logs_message);
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            Subscription::run(network_worker).map(Message::Network),
        ])
    }

    pub fn view(&self) -> Element<Message> {
        let navbar = self.navbar.view().map(Message::Navbar);

        let content = if self.navbar.connection_state() == ConnectionState::Connected {
            container(match self.navbar.active_tab() {
                Tab::Map => self.map_view.view(&self.game_state).map(Message::Map),
                Tab::Settings => self.settings_view.view().map(Message::Settings),
                Tab::Logs => self.logs_view.view().map(Message::Logs),
            })
        } else {
            container(text("Please connect to the server to view the content.").size(24))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(bordered_box)
        };

        let footer = self.footer.view(&self.game_state).map(Message::Footer);

        column![
            navbar,
            content.width(Length::Fill).height(Length::Fill),
            footer
        ]
        .into()
    }
}
