use crate::Tab;
use iced::alignment::Vertical;
use iced::widget::{button, container, row, text_input, vertical_rule};
use iced::{Element, Length, Padding, Pixels};

#[derive(Debug, Clone)]
pub enum NavbarMessage {
    TabSelected(Tab),
    ChangeIp(String),
    ChangePort(String),
    Connect(String, String),
}

pub struct Navbar {
    active_tab: Tab,
    pub ip: String,
    pub port: String,
}

impl Default for Navbar {
    fn default() -> Self {
        Self {
            active_tab: Tab::default(),
            ip: String::from("127.0.0.1"),
            port: String::from("4242"),
        }
    }
}

impl Navbar {
    pub fn update(&mut self, message: NavbarMessage) {
        match message {
            NavbarMessage::TabSelected(tab) => {
                self.active_tab = tab;
            }
            NavbarMessage::ChangeIp(content) => {
                self.ip = content;
            }
            NavbarMessage::ChangePort(content) => {
                self.port = content;
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<NavbarMessage> {
        let ip_input = text_input("IP", &self.ip)
            .on_input(NavbarMessage::ChangeIp)
            .width(Length::FillPortion(3));
        let port_input = text_input("Port", &self.port)
            .on_input(NavbarMessage::ChangePort)
            .width(Length::FillPortion(1));
        let login_button = button("Login")
            .style(button::primary)
            .width(Length::Shrink)
            .on_press(NavbarMessage::Connect(self.ip.clone(), self.port.clone()));

        fn create_tab_button(
            label: &str,
            tab: Tab,
            active_tab: Tab,
        ) -> iced::widget::Button<'_, NavbarMessage> {
            if tab == active_tab {
                button(label).style(button::primary)
            } else {
                button(label)
                    .on_press(NavbarMessage::TabSelected(tab))
                    .style(button::primary)
            }
        }

        let tab_canvas = create_tab_button("Map", Tab::Map, self.active_tab);
        let tab_settings = create_tab_button("Settings", Tab::Settings, self.active_tab);
        let tab_info = create_tab_button("Logs", Tab::Logs, self.active_tab);

        container(
            row![
                ip_input,
                port_input,
                login_button,
                vertical_rule(5),
                tab_canvas,
                tab_settings,
                tab_info
            ]
            .spacing(Pixels::from(10))
            .padding(Padding::from([0, 10]))
            .align_y(Vertical::Center),
        )
        .center(Length::Fill)
        .height(40)
        .style(container::rounded_box)
        .into()
    }

    pub fn active_tab(&self) -> Tab {
        self.active_tab
    }
}
