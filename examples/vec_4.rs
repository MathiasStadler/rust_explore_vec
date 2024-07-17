fn main (){

    // FROM HERE
    // https://doc.rust-lang.org/std/vec/struct.Vec.html

    // FROM HERE
    // https://dhghomon.github.io/easy_rust/Chapter_21.html

    let mut vec_instances = Vec::new();

    let mut vec_str = Vec::new();
    vec_str.push(String::from("first"));
    vec_str.push(String::from("\"second\""));
    vec_str.push(String::from("third"));
    vec_str.push(String::from("//child")); 

    vec_instances.push(vec_str.clone());


    // for x in &vec_str {
    //     println!("{x}");
    // }

    // A `Vector` can also be iterated over while the iteration
    // count is enumerated in a separate variable (`i`)
    //FROM HERE => https://doc.rust-lang.org/rust-by-example/std/vec.html
    for (i, x) in vec_instances.iter().enumerate() {
        println!("vec_instances => In position {:?} we have value {:?}", i, x);
        println!("vec_instances => In position {:?} we have value {:?}", i, x[0]);
        println!("vec_instances => In position {:?} we have value {:?}", i, x[1]);
    }

    for (i, x) in vec_str.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

}