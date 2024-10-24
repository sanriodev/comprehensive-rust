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