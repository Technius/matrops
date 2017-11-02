use cursive::Cursive;
use cursive::view::{Offset, Position};
use cursive::views;

use ui::open_error_popup;

use std::str::FromStr;
use std::marker::PhantomData;

pub struct NumberDialog<T, F> {
    pub message: String,
    pub callback: F,
    phantom: PhantomData<T>
}

impl <T: 'static + FromStr + Copy + Clone, F: 'static + Fn(&mut Cursive, T)> NumberDialog<T, F> {
    pub fn new<S: Into<String>>(message: S, callback: F) -> Self {
        NumberDialog {
            message: message.into(),
            callback: callback,
            phantom: PhantomData
        }
    }

    pub fn show(self, s: &mut Cursive) {
        let msg = self.message.clone();
        let edit_text = views::EditView::new()
            .on_submit(move |s, txt| {
                match T::from_str(txt) {
                    Ok(n) => {
                        s.pop_layer();
                        (self.callback)(s, n);
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
}
