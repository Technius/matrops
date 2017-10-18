extern crate matrixops;

use matrixops::matrix::Matrix;
use matrixops::ui::command::Command;

#[test]
fn command_add() {
    let d = vec![1, -1 ,0,
                 0,  1, 0,
                 0,  0, 1
    ];
    let m1 = Matrix::new(3, 3, d.clone());
    let cmd = Command::AddRow {
        coeff: 1,
        src: 2,
        dest: 1
    };
    assert_eq!(cmd.apply(&m1).unwrap(), Matrix::new(3, 3, vec![
        1, 0, 0,
        0, 1, 0,
        0, 0, 1
    ]));
}

#[test]
fn command_swap() {
    let d = vec![0, 1 ,0,
                 1, 0, 0,
                 0, 0, 1
    ];
    let m1 = Matrix::new(3, 3, d.clone());
    let cmd = Command::SwapRow {
        row1: 1,
        row2: 2
    };
    assert_eq!(cmd.apply(&m1).unwrap(), Matrix::new(3, 3, vec![
        1, 0, 0,
        0, 1, 0,
        0, 0, 1
    ]));
}

#[test]
fn command_scale() {
    let d = vec![2.0, 0.0 ,0.0,
                 0.0, 1.0, 0.0,
                 0.0, 0.0, 1.0
    ];
    let m1 = Matrix::new(3, 3, d.clone());
    let cmd = Command::ScaleRow {
        coeff: 0.5,
        row: 1
    };
    assert_eq!(cmd.apply(&m1).unwrap(), Matrix::new(3, 3, vec![
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0
    ]));
}
