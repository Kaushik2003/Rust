// #[derive(Debug)]//
// struct Rectangle{
//     width:u32,
//     height:u32
// }


// fn main() {
//     let rect=Rectangle{
//         width:50,
//         height:32
//     };

//     println!(
//         "The area of the rectangle that is {:?} is {} square pixels.",rect,area(&rect)
//     );
// }

// fn area(rect: &Rectangle)-> u32 {
//     rect.width * rect.height
// }



#[derive(Debug)]//
struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle{
    fn area(&self)->u32{
        self .height * self.width
    }
}

fn main() {
    let rect=Rectangle{
        width:50,
        height:32
    };

    println!(
        "The area of the rectangle that is {:?} is {} square pixels.",rect,rect.area()
    );
}

fn langda(){
    
}