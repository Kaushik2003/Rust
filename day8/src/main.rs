// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let mut user_1 = User {
//         active: true,
//         username: String::from("kaushiksamadder"),
//         email: String::from("kaushiksamadder2003@gmail.com"),
//         sign_in_count: 0,
//     };
//     user_1.username = String::from("Not Available");
//     user_1.username.push_str(" YOOOOOO");

//     println!("THe new username is {}", user_1.username);

//     let s1 = user_1.username;
//     // print!("User name : {}",user_1.username);    value is moved to
//     println!("User name : {}", s1);
    
//     let user2=build_user(
//         String::from("kzark2003"),String::from("haha15032003@gmail.com"));

//     println!("The name of user 2 is {}",user2.username);
// }

// fn build_user(username: String,email:String)->User{
//     User {
//         username,
//         email,
//         active:true,
//         sign_in_count:0,
//     }
// }





//* Using tuple structs without named fields to create different type*/
struct Color(u8,u8,u8);
struct Point(u8,u8,u8);
fn main(){
    let red:Color=Color(100,152,0);
    set(red);
    let p:Point=Point(0,120,90);
    move_bg(p);
} 
fn set(color:Color){
    println!("setting the color R={},G={},B={}",color.0,color.1,color.2);
}

fn move_bg(point: Point){
    println!("Moving from 0,0,0 to X={},Y={},Z={}",point.0,point.1,point.2);
}
