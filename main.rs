struct Rectangle{
    width:u32,
    height:u32,
}
fn area(rec: &Rectangle)-> u32{
    rec.width*rec.height
}
fn main()
{
    let a = Rectangle{
        width:34,
        height:23,
            
        };      
    
    println!("the area of {},{} is {}",a.width,a.height,area(&a));
}