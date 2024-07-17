fn main (){

    // FROM HERE
    // https://doc.rust-lang.org/std/vec/struct.Vec.html

    let mut vec = Vec::new();
    vec.push("first");
    vec.push("second"); 

    for x in &vec {
        println!("{x}");
    }

}