use termion::{style, terminal_size};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::Item;

pub fn draw(items: &[Item]) {
    let width = terminal_size().unwrap().0 as usize;

    if !items.is_empty() {
        println!("+{}+", "-".repeat(width - 2));
    }
    // print name
    for item in items.iter() {
        println!("|{}|", " ".repeat(width - 2));
        println!(
            "| {}{}{}{}|",
            style::Bold,
            item.name,
            " ".repeat(width - item.name.width() - 3),
            style::Reset
        );
        println!("+-------+{}+", "-".repeat(width - 10));

        // print description
        if let Some(desc) = &item.desc {
            println!(
                "| Desc  | {}{}|",
                desc,
                " ".repeat(width - desc.width() - 11)
            );
            println!("+-------+{}+", "-".repeat(width - 10));
        }

        // print about
        if let Some(about) = &item.about {
            let mut cs = about.chars();
            let mut flag = false;
            let mut idx = 0;
            let label = ["About", "     "];
            loop {
                let mut line = vec![];
                let mut len = 0;

                while len < width - 14 {
                    if let Some(c) = cs.next() {
                        len += c.width().unwrap_or_default();
                        line.push(c);
                    } else {
                        flag = true;
                        break;
                    }
                }
                let line: String = line.iter().collect();

                println!(
                    "| {} | {}{} |",
                    label[idx],
                    line,
                    " ".repeat(width - line.width() - 12)
                );

                idx = 1;
                if flag {
                    break;
                }
            }
            println!("+-------+{}+", "-".repeat(width - 10));
        }
        println!(
            "| URL   | {}{}|",
            item.url,
            " ".repeat(width - item.url.width() - 11)
        );
        println!("+-------+{}+", "-".repeat(width - 10));
    }
}
