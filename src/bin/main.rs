extern crate matrixops;
extern crate cursive;
extern crate num_rational;
extern crate num_traits;

use std::str::FromStr;

use cursive::Cursive;
use cursive::views;
use cursive::view::{Offset, Position};
use cursive::traits::Identifiable;

use matrixops::matrix::Matrix;
use matrixops::ui::MatrixView;
use matrixops::ui::command::Command;

use num_rational::Ratio;
use num_traits::FromPrimitive;

fn main() {
    let mut siv = Cursive::new();

    let status_bar = views::BoxView::with_full_width(views::TextView::new("Status"));
    let screen_size = siv.screen_size();
    siv.screen_mut().add_layer_at(Position::new(
        Offset::Absolute(screen_size.x), Offset::Absolute(screen_size.y + 1)), status_bar);

    show_setup_view(&mut siv);
    siv.run();

}

fn show_setup_view(s: &mut Cursive) {
    let row_text = views::EditView::new().with_id("rows");
    let col_text = views::EditView::new().with_id("columns");
    let setup_pane = views::ListView::new()
        .child("Rows", row_text)
        .child("Columns", col_text);
    let setup_dialog = views::Dialog::around(setup_pane)
        .title("Enter matrix dimensions")
        .button("Go", |s| {
            let rt = s.find_id::<views::EditView>("rows").expect("Can't find row EditText");
            let ct = s.find_id::<views::EditView>("columns").expect("Can't find col EditText");
            let rows = usize::from_str(&*rt.get_content());
            let cols = usize::from_str(&*ct.get_content());

            let mut show_error = true;
            if let (Ok(rows), Ok(cols)) = (rows, cols) {
                if rows > 0 && cols > 0 {
                    let data = vec![Ratio::from_i64(0).unwrap(); rows * cols];
                    let matrix = Matrix::new(rows, cols, data);
                    s.pop_layer();
                    show_edit_view(s, matrix);
                    show_error = false;
                }
            }

            if show_error {
                open_error_popup(s, "Please enter positive integers");
            }
        });
    s.add_layer(setup_dialog);
}

fn show_edit_view(s: &mut Cursive, data: Matrix<Ratio<i64>>) {
    let mview = MatrixView::<Ratio<i64>>::new(data).with_id("matrix_view");
    let scale_button = views::Button::new("Scale row", scale_action);
    let swap_button = views::Button::new("Swap rows", swap_action);
    let add_button = views::Button::new("Add rows", add_action);
    let layout = views::LinearLayout::vertical()
        .child(mview)
        .child(scale_button)
        .child(swap_button)
        .child(add_button);
    let diag = views::Dialog::new()
        .content(layout)
        .title("MatrixOps")
        .button("Quit", |s| s.quit());
    let eview = views::OnEventView::new(diag)
        .on_event('s', scale_action)
        .on_event('i', swap_action)
        .on_event('a', add_action);
    s.add_layer(eview);
}

fn scale_action(s: &mut Cursive) {
    open_number_dialog(s, "Which row?", |s, row: usize| {
        open_number_dialog(s, "How much to scale by?", move |s, coeff: Ratio<i64>| {
            s.call_on_id("matrix_view", |view: &mut MatrixView<Ratio<i64>>| {
                // FIXME: error dialog
                let _ = view.apply_command(Command::ScaleRow {
                    coeff: coeff,
                    row: row
                });
            });
        });
    });
}

fn swap_action(s: &mut Cursive) {
    open_number_dialog(s, "First row?", |s, row1: usize| {
        open_number_dialog(s, "Second row?", move |s, row2: usize| {
            s.call_on_id("matrix_view", |view: &mut MatrixView<Ratio<i64>>| {
                // FIXME: error dialog
                let _ = view.apply_command(Command::SwapRow {
                    row1: row1,
                    row2: row2
                });
            });
        });
    });
}

fn add_action(s: &mut Cursive) {
    open_number_dialog(s, "Source row?", |s, src: usize| {
        open_number_dialog(s, "Multiplied by?", move |s, coeff: Ratio<i64>| {
            open_number_dialog(s, "Dest row?", move |s, dest: usize| {
                s.call_on_id("matrix_view", |view: &mut MatrixView<Ratio<i64>>| {
                    // FIXME: error dialog
                    let _ = view.apply_command(Command::AddRow {
                        src: src,
                        coeff: coeff,
                        dest: dest
                    });
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
