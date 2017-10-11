pub mod command;

use cursive;
use cursive::view::{Finder, ViewWrapper};
use cursive::views;
use cursive::align::HAlign;
use cursive::traits::{Identifiable, View};
use std;

use matrix::Matrix;

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
                let cell = views::TextView::new(Self::cell_text(value, max_width))
                    .h_align(HAlign::Right)
                    .with_id(Self::cell_id(row_ind, col_ind));
                rview.add_child(views::DummyView {});
                rview.add_child(cell);
            }
            row_views.add_child(rview);
        }

        MatrixView {
            matrix: matrix,
            underlying: row_views
        }
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
