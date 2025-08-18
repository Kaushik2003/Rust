struct Rectangle{
    width:u32,
    height:u32
}

fn main() {
    let rect=Rectangle{
        width:50,
        height:32
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect)
    );
}

fn area(rect: Rectangle)-> u32 {
    rect.width * rect.height
}