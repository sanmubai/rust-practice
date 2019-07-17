#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height 
    }
}


fn main() {
    println!("struct_rectangle");

    let rect1=Rectangle{width:32,height:44};
    println!("rect1:{:#?}",rect1);
    println!("rect area {}",rect1.area());

}
