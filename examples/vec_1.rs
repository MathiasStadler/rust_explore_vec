fn main (){

    // FROM HERE
    // https://doc.rust-lang.org/std/vec/struct.Vec.html

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2); 

    for x in &vec {
        println!("{x}");
    }

}