use array_handling::{array_intro, transpose, tuples_instro};

mod array_handling;

fn main() {
    array_intro();

    tuples_instro();

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}
