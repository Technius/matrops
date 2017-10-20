pub mod command;

use cursive;
use cursive::Cursive;
use cursive::view::{Finder, ViewWrapper, Offset, Position};
use cursive::views;
use cursive::align::HAlign;
use cursive::traits::{Identifiable, View};
use std;
use std::str::FromStr;
use std::ops::{Add, Mul};

use matrix::{Matrix, MatrixResult};
use self::command::Command;

pub struct MatrixView<T> {
    pub matrix: Matrix<T>,
    underlying: views::LinearLayout
}

impl <T: Clone + std::string::ToString> MatrixView<T> {
    pub fn new(matrix: Matrix<T>) -> Self {
        let mut row_views = views::LinearLayout::vertical();
        let max_width = Self::max_cell_size(&matrix);

        for (row_ind, row) in matrix.rows().iter().enumerate() {
            let mut rview = views::LinearLayout::horizontal();
            for (col_ind, value) in row.iter().enumerate() {
                let cell_id = Self::cell_id(row_ind, col_ind);
                let cell = views::TextView::new(Self::cell_text(value, max_width))
                    .h_align(HAlign::Right)
                    .with_id(cell_id.as_str());
                let event_view = views::OnEventView::new(cell)
                    .on_event('e', move |s| Self::show_cell_edit_popup(s, &cell_id));
                rview.add_child(views::DummyView {});
                rview.add_child(event_view);
            }
            row_views.add_child(rview);
        }

        MatrixView {
            matrix: matrix,
            underlying: row_views
        }
    }

    pub fn apply_command(&mut self, cmd: Command<T>) -> MatrixResult<()>
        where T: Add<T, Output = T> + Mul<T, Output = T> {
        let upd = cmd.apply(&self.matrix)?;
        self.matrix = upd;
        Ok(())
    }

    fn show_cell_edit_popup(s: &mut Cursive, cell_id: &str) {
        s.call_on_id(cell_id, |et: &mut views::EditView| {
        });
    }

    fn max_cell_size(matrix: &Matrix<T>) -> usize {
        matrix.rows().iter()
            .flat_map(|r| r)
            .map(|v| v.to_string().len() as usize)
            .max()
            .unwrap()
    }

    fn cell_text(value: &T, max_size: usize) -> String {
        let mut s = value.to_string();
        let padding = std::iter::repeat(" ")
            .take(max_size - s.len())
            .collect::<String>();
        s.push_str(&padding);
        s
    }

    fn update(&mut self) {
        let mat = &self.matrix;
        let max_width = Self::max_cell_size(mat);
        for (row_ind, row) in mat.rows().iter().enumerate() {
            for (col_ind, value) in row.iter().enumerate() {
                let id = Self::cell_id(row_ind, col_ind);
                self.underlying.find_id(&id, |view: &mut views::TextView| {
                    view.set_content(Self::cell_text(value, max_width));
                });
            }
        }
    }

    fn cell_id(row_ind: usize, col_ind: usize) -> String {
        format!("cell_{},{}", row_ind + 1, col_ind + 1)
    }
}

impl <T: Clone + ToString> ViewWrapper for MatrixView<T> {
    type V = views::LinearLayout;

    fn wrap_layout(&mut self, size: cursive::vec::Vec2) -> () {
        self.update();
        self.with_view_mut(|v| v.layout(size));
    }

    fn with_view<F, R>(&self, f: F) -> Option<R>
        where F: FnOnce(&Self::V) -> R {
        Some(f(&self.underlying))
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
        where F: FnOnce(&mut Self::V) -> R {
        Some(f(&mut self.underlying))
    }
}

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
