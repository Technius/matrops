extern crate matrixops;
extern crate cursive;

use cursive::Cursive;
use cursive::views;
use cursive::view::{Offset, Position};
use cursive::traits::Identifiable;

use matrixops::matrix::Matrix;
use matrixops::ui::MatrixView;

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
    let layout = views::LinearLayout::vertical()
        .child(mview)
        .child(mx2button);
    let diag = views::Dialog::new()
        .content(layout)
        .title("MatrixOps")
        .button("Quit", |s| s.quit());
    let eview = views::OnEventView::new(diag).on_event('a', open_number_dialog);
    siv.add_layer(eview);
    siv.run();
}

fn open_number_dialog(s: &mut Cursive) {
    let popup = views::Dialog::text("Hello!").dismiss_button("Close me");
    s.screen_mut().add_layer_at(Position::new(Offset::Center, Offset::Parent(10)), popup);
}

fn open_error_popup(s: &mut Cursive, msg: String) {
    let popup = views::Dialog::text(format!("Error: {}", msg)).dismiss_button("Close");
    s.screen_mut().add_layer_at(Position::new(Offset::Center, Offset::Center), popup);
}
