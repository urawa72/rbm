use crate::bookmarks::Bookmark;
use skim::prelude::*;
use skim::{Skim, SkimItemReceiver, SkimItemSender};
use std::{process::Command, sync::Arc};

impl SkimItem for Bookmark {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.inner)
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.url)
    }
}

/// Search bookmark with fuzzy finder.
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
        // Only for Mac
        // TODO: Open browser on linux
        Command::new("open")
            .arg(url.as_ref())
            .output()
            .expect("Faild to execute process");
    }
}
