extern crate matrixops;
extern crate cursive;

use cursive::Cursive;
use cursive::views;
use cursive::view::{Finder, ViewWrapper};
use cursive::traits::{Identifiable, View};

use matrixops::matrix::Matrix;

fn main() {
    let mut siv = Cursive::new();
    let mview = MatrixView::new(Matrix::new(3, 3, vec![
        3, 2, 11,
        9, -10, 3,
        42, 9, 0
    ])).with_id("matrix_view");
    let mx2button = views::Button::new("All rows *2", |s| {
        s.call_on_id("matrix_view", |view: &mut MatrixView| {
            let mat = &mut view.matrix;
            for row in 1..(mat.rows + 1) {
                mat.row_foreach(row, |x| x * 2);
            }
        });
    });
    let layout =views::LinearLayout::vertical()
        .child(mview)
        .child(mx2button);
    let diag = views::Dialog::new()
        .content(layout)
        .title("MatrixOps")
        .button("Quit", |s| s.quit());
    siv.add_layer(diag);
    siv.run();
}

struct MatrixView {
    pub matrix: Matrix,
    underlying: views::LinearLayout
}

impl MatrixView {
    fn new(matrix: Matrix) -> Self {
        let mut row_views = views::LinearLayout::vertical();
        let max_width = Self::max_cell_size(&matrix);

        for (row_ind, row) in matrix.rows().iter().enumerate() {
            let mut rview = views::LinearLayout::horizontal();
            for (col_ind, value) in row.iter().enumerate() {
                let cell = views::TextView::new(Self::cell_text(value, max_width))
                    .h_align(cursive::align::HAlign::Right)
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

    fn max_cell_size(matrix: &Matrix) -> usize {
        matrix.rows().iter()
            .flat_map(|r| r)
            .map(|v| v.to_string().len() as usize)
            .max()
            .unwrap()
    }

    fn cell_text(value: &i32, max_size: usize) -> String {
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

impl ViewWrapper for MatrixView {
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
