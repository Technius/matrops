#[macro_use]
extern crate matrixops;
extern crate cursive;

use cursive::Cursive;

#[test]
fn compile_test() {
    let s: &mut Cursive = &mut Cursive::new();
    number_dialog_chain!(s, {
        a: usize =? "get a";
        b: usize =? "get b";
        callback {
            let _ = s;
            a + b + 1
        }
    })
}
