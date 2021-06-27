fn main() {

    let a=123;
    println!("Hello, world! {}",a);

    let b =32;
    println!("hello 2{}", b);

    let mut s=String::from("hello");
    let s2=&mut s;
    s2.push_str("obb");
    s.push_str("bb2");
    println!("{}",s)
}
