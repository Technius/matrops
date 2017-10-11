extern crate matrixops;

use matrixops::matrix;

#[test]
fn matrix_add() {
    let d = vec![1, 0 ,0,
                 0, 1, 0,
                 0, 0, 1
    ];
    let m1 = matrix::Matrix::new(3, 3, d.clone());
    let m2 = matrix::Matrix::new(3, 3, d.clone());
    let m3 = matrix::Matrix::new(3, 3, vec![
        2, 0, 0,
        0, 2, 0,
        0, 0, 2
    ]);
    assert_eq!(&m1 + &m2, m3);
}

#[test]
fn matrix_row_set() {
    let d = vec![1, 0 ,0,
                 0, 1, 0,
                 0, 0, 1
    ];
    let mut m1 = matrix::Matrix::new(3, 3, d.clone());
    m1.set_row(1, &[1, 2, 3]);
    assert_eq!(m1, matrix::Matrix::new(3, 3, vec![
        1, 2, 3,
        0, 1, 0,
        0, 0, 1
    ]));
}

#[test]
fn matrix_row_foreach() {
    let d = vec![1, 0 ,0,
                 0, 1, 0,
                 0, 0, 1
    ];
    let mut m1 = matrix::Matrix::new(3, 3, d.clone());
    m1.row_foreach(1, |x| x + 1);
    assert_eq!(m1, matrix::Matrix::new(3, 3, vec![
        2, 1, 1,
        0, 1, 0,
        0, 0, 1
    ]));

    let mut m2 = matrix::Matrix::new(3, 3, d.clone());
    m2.row_foreach(2, |x| x * 2);
    assert_eq!(m2, matrix::Matrix::new(3, 3, vec![
        1, 0, 0,
        0, 2, 0,
        0, 0, 1
    ]));
}
