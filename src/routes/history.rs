use iced::widget::{column, container, scrollable};
use iced::Element;
use iced::{Length, Padding};

use crate::components::{h_header, h_transaction_item};
use crate::{HarborWallet, Message};

pub fn history(harbor: &HarborWallet) -> Element<Message> {
    let header = h_header("History", "Here's what's happened so far.");

    let transactions = harbor
        .transaction_history
        .iter()
        .fold(column![], |column, item| {
            column.push(h_transaction_item(item))
        });

    let column = column![header, transactions].spacing(48);

    container(scrollable(
        column
            .spacing(48)
            .width(Length::Fill)
            .max_width(512)
            .padding(Padding::new(48.)),
    ))
    .into()
}