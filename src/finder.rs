use skim::prelude::*;
use skim::{Skim, SkimItemReceiver, SkimItemSender};
use std::{
    process::Command,
    sync::Arc,
};
use crate::bookmarks::Bookmark;

impl SkimItem for Bookmark {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.inner)
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.url)
    }
}

pub fn finder(bookmarks: Vec<Bookmark>) {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .bind(vec!["Enter:accept"])
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for bookmark in bookmarks {
        let _ = tx_item.send(Arc::new(bookmark));
    }
    drop(tx_item);

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| match out.final_key {
            Key::Enter => out.selected_items,
            _ => Vec::new(),
        })
        .unwrap_or_else(Vec::new);

    for item in selected_items.iter() {
        let url = item.output();
        // only for Mac
        Command::new("open")
            .arg(url.as_ref())
            .output()
            .expect("faild to execute process");
    }
}
