fn main() {
    let mut v = vec![1, 2, 3];
    for i in 0..v.len() {
        v[i] = v[i] * 2; //Modify in a safe manner
    }
    println!("{:?}", v);
}