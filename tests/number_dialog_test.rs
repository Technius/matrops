extern crate matrixops;
extern crate cursive;

use cursive::Cursive;

use matrixops::ui::NumberDialog;

#[test]
fn compile_test() {
    let nd = NumberDialog::new("foo", |_, value: i32| {
    });
}
