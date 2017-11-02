pub mod command;
mod matrix_view;
mod number_dialog;

pub use self::matrix_view::MatrixView;
pub use self::number_dialog::NumberDialog;

use cursive::Cursive;
use cursive::view::{Offset, Position};
use cursive::views;
use std;
use std::str::FromStr;

/// Opens a dialog that prompts for a number, and then calls the callback
/// with the entered number.
/// FIXME: Write a wrapper that makes chaining easier
pub fn open_number_dialog<F, S: Into<String>, T>(s: &mut Cursive, msg: S, callback: F)
    where F: 'static + Fn(&mut Cursive, T),
          T: FromStr + Copy + Clone {
    let edit_text = views::EditView::new()
        .on_submit(move |s, txt| {
            match T::from_str(txt) {
                Ok(n) => {
                    s.pop_layer();
                    callback(s, n);
                },
                Err(_) => {
                    open_error_popup(s, "Please enter a number.");
                }
            }
        });
    let popup = views::Dialog::around(edit_text)
        .title(msg)
        .dismiss_button("Cancel");
    s.screen_mut().add_layer_at(Position::new(Offset::Center, Offset::Parent(10)), popup);
}

pub fn open_error_popup<S: std::fmt::Display>(s: &mut Cursive, msg: S) {
    let popup = views::Dialog::text(format!("Error: {}", msg)).dismiss_button("Close");
    s.screen_mut().add_layer_at(Position::new(Offset::Center, Offset::Center), popup);
}
