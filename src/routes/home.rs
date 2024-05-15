use crate::components::{h_button, SvgIcon};
use iced::widget::{center, column, container, row, text};
use iced::{Alignment, Element};

use crate::{HarborWallet, Message};

use super::Route;

pub fn home(harbor: &HarborWallet) -> Element<Message> {
    let balance = text(format!("{} sats", harbor.balance_sats)).size(64);
    let send_button =
        h_button("Send", SvgIcon::UpRight, false).on_press(Message::Navigate(Route::Send));
    let receive_button =
        h_button("Receive", SvgIcon::DownLeft, false).on_press(Message::Navigate(Route::Receive));
    let buttons = row![send_button, receive_button].spacing(32);

    container(center(
        column![balance, buttons]
            .spacing(32)
            .align_items(Alignment::Center)
            .max_width(512),
    ))
    .into()
}
