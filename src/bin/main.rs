#[macro_use]
extern crate matrixops;
extern crate cursive;
extern crate num_rational;
extern crate num_traits;

use cursive::Cursive;
use cursive::view::{Offset, Position};
use cursive::views;
use cursive::traits::Identifiable;
use num_rational::Ratio;
use num_traits::FromPrimitive;
use std::cell::RefCell;
use std::str::FromStr;

use matrixops::matrix::Matrix;
use matrixops::ui::{open_error_popup, open_number_dialog, MatrixView};
use matrixops::ui::command::Command;

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
        .on_event('a', add_action)
        .on_event('e', edit_action)
        .on_event('E', edit_row_action);
    s.add_layer(eview);
}

fn scale_action(s: &mut Cursive) {
    number_dialog_chain!(s, {
        row: usize =? "Which row?";
        coeff: Ratio<i64> =? "How much to scale by?";
        s.call_on_id("matrix_view", |view: &mut MatrixView<Ratio<i64>>| {
            // FIXME: error dialog
            let _ = view.apply_command(Command::ScaleRow {
                coeff: coeff,
                row: row
            });
        });
    });
}

fn swap_action(s: &mut Cursive) {
    number_dialog_chain!(s, {
        row1: usize =? "First row?";
        row2: usize =? "Second row?";
        s.call_on_id("matrix_view", |view: &mut MatrixView<Ratio<i64>>| {
            // FIXME: error dialog
            let _ = view.apply_command(Command::SwapRow {
                row1: row1,
                row2: row2
            });
        });
    });
}

fn edit_action(s: &mut Cursive) {
    number_dialog_chain!(s, {
        row: usize =? "Row?";
        col: usize =? "Col?";
        v: Ratio<i64> =? "New value?";
        s.call_on_id("matrix_view", |view: &mut MatrixView<Ratio<i64>>| {
            let _ = view.apply_command(Command::EditCell {
                row,
                col,
                value: v
            });
        });
    });
}

fn edit_row_action(s: &mut Cursive) {
    open_number_dialog(s, "Row?", |s: &mut Cursive, row: usize| {
        let mview = s.find_id::<MatrixView<Ratio<i64>>>("matrix_view")
            .expect("matrix view not found");

        if row < mview.matrix.rows {
            // Ok, we need to open a dialog for each index in the row, one after
            // another. This is hard to do with a for loops, so we'll use a
            // recursive function instead. Closures can't be recursive, so
            // instead we'll use a function and pass in the "environment"
            // manually.

            // Initialize the list of new values, and put it in a `RefCell` so
            // it can be mutated across calls to the recursive function.
            let row_rc = RefCell::new(mview.matrix.get_row(row));

            // The struct that will hold values belonging to the function's
            // "environment"
            #[derive(Clone, Copy)]
            struct Env {
                max_index: usize,
                row: usize
            }

            fn go(env: Env, s: &mut Cursive, rc: RefCell<Vec<Ratio<i64>>>, index: usize) {
                if index <= env.max_index {
                    // We're going to modify the values

                    let label = format!("Cell {}?", index + 1);
                    open_number_dialog(s, label, move |s: &mut Cursive, value: Ratio<i64>| {
                        rc.borrow_mut()[index] = value;
                        go(env, s, rc.clone(), index + 1);
                    });
                } else {
                    // All of the values have been read, so we can now apply the
                    // action

                    let new_values: Vec<Ratio<i64>> = rc.into_inner();
                    s.call_on_id("matrix_view", |view: &mut MatrixView<Ratio<i64>>| {
                        let _ = view.apply_command(Command::EditRow {
                            row: env.row,
                            values: new_values
                        });
                    });
                }
            }

            let env = Env {
                max_index: mview.matrix.cols - 1,
                row: row
            };
            go(env, s, row_rc.clone(), 0);
        }
    });
}

fn add_action(s: &mut Cursive) {
    number_dialog_chain!(s, {
        src: usize =? "Source row?";
        coeff: Ratio<i64> =? "Multiplied by?";
        dest: usize =? "Dest row?";
        s.call_on_id("matrix_view", |view: &mut MatrixView<Ratio<i64>>| {
            // FIXME: error dialog
            let _ = view.apply_command(Command::AddRow {
                src: src,
                coeff: coeff,
                dest: dest
            });
        });
    });
}

