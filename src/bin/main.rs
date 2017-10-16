extern crate matrixops;
extern crate cursive;

use std::str::FromStr;

use cursive::Cursive;
use cursive::views;
use cursive::view::{Offset, Position};
use cursive::traits::Identifiable;

use matrixops::matrix::Matrix;
use matrixops::ui::MatrixView;
use matrixops::ui::command::Command;

fn main() {
    let mut siv = Cursive::new();

    let status_bar = views::BoxView::with_full_width(views::TextView::new("Status"));
    let screen_size = siv.screen_size();
    siv.screen_mut().add_layer_at(Position::new(
        Offset::Absolute(screen_size.x), Offset::Absolute(screen_size.y + 1)), status_bar);

    let mview = MatrixView::<f64>::new(Matrix::new(3, 3, vec![
        3.0, 2.0, 11.0,
        9.0, -10.0, 3.0,
        42.0, 9.0, 0.0]))
        .with_id("matrix_view");
    let mx2button = views::Button::new("All rows *2", |s| {
        s.call_on_id("matrix_view", |view: &mut MatrixView<f64>| {
            let mat = &mut view.matrix;
            for row in 1..(mat.rows + 1) {
                mat.row_foreach(row, |x| x * 2.0).unwrap();
            }
        });
    });
    let scale_button = views::Button::new("Scale row", scale_action);
    let layout = views::LinearLayout::vertical()
        .child(mview)
        .child(mx2button)
        .child(scale_button);
    let diag = views::Dialog::new()
        .content(layout)
        .title("MatrixOps")
        .button("Quit", |s| s.quit());
    let eview = views::OnEventView::new(diag).on_event('s', scale_action);
    siv.add_layer(eview);
    siv.run();
}

fn scale_action(s: &mut Cursive) {
    open_number_dialog(s, "How much to scale by?", |s, coeff: f64| {
        open_number_dialog(s, "Which row?", move |s, row: usize| {
            s.call_on_id("matrix_view", |view: &mut MatrixView<f64>| {
                // FIXME: error dialog
                let _ = view.apply_command(Command::ScaleRow {
                    coeff: coeff,
                    row: row
                });
            });
        });
    });
}

/// Opens a dialog that prompts for a number, and then calls the callback
/// with the entered number.
/// FIXME: Write a wrapper that makes chaining easier
fn open_number_dialog<F, S: Into<String>, T>(s: &mut Cursive, msg: S, callback: F)
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

fn open_error_popup<S: std::fmt::Display>(s: &mut Cursive, msg: S) {
    let popup = views::Dialog::text(format!("Error: {}", msg)).dismiss_button("Close");
    s.screen_mut().add_layer_at(Position::new(Offset::Center, Offset::Center), popup);
}
