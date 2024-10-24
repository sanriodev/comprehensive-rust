use std::ptr::null;

pub fn array_intro() {
    let arr: [i32; 2] = [2,15]; // array type declaration -> [type; size] -> [i32; 10]
    //length also matters for type, which means two int-arrays of different lengths are different types
    // [i32; 10] != [i32; 20]
    println!("Array: {:?}", arr);
    println!("Array with pretty print: {a:#?}", a=arr);
}

pub fn tuples_instro() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", tup);
    println!("Tuple with pretty print: {t:#?}", t=tup);
    let (x, y, z) = tup;
    println!("Tuple destructuring: x: {}, y: {}, z: {}", x, y, z);
    println!("Tuple indexing: x: {}, y: {}, z: {}", tup.0, tup.1, tup.2);
}

pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let start_matrix = matrix;
    let mut finished: [[i32; 3]; 3] = [[0; 3]; 3];

    for i in 0..start_matrix.len() {
        for j in 0..start_matrix[i].len() {
            finished[j][i] = start_matrix[i][j];
        }
    }
    return finished;
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}
