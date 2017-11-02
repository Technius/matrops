pub mod command;
mod matrix_view;

pub use self::matrix_view::MatrixView;

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

/// A small DSL for chaining multiple number dialogs.
///
/// Example:
/// ```
/// use cursive::Cursive;
/// use cursive::views::Dialog;
///
/// let s: &mut Cursive = &mut Cursive::new();
/// number_dialog_chain!(s, {
///     a: usize =? "Prompt 1"
///     b: usize =? "Prompt 2"
///     callback {
///         s.add_layer(Dialog::text(format!("a: {}, b: {}", a, b)).dismiss_button("Close"));
///     }
/// });
/// ```
#[macro_export]
macro_rules! number_dialog_chain {
    ($s:ident, { $name:ident : $ty:ty =? $msg:expr; $($t: tt)* }) => {
        matrixops::ui::open_number_dialog($s, $msg, move |$s: &mut Cursive, $name: $ty| {
            number_dialog_chain!($s, { $($t)* })
        });
    };
    ($s:ident, { callback $stats:block }) => {{ $stats; () }};
    ($s:ident, ) => ()
}
