#[derive(Debug)]
enum IndState{
    Delhi,
    WestBengal,
    Maharashtra, 
}
#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(IndState),
}
fn main() {
let coin=Coin::Quarter(IndState::WestBengal);
println!("Value is {}",value_in_cent(coin));

}

fn value_in_cent(coin:Coin)->u8{
    match coin{
        Coin::Penny=>1,
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quarter(IndState::WestBengal)=>{
            println!("Hello from West Bengal");
            25
        }
        Coin::Quarter(state)=>{
            println!("Got Q of value {:?}",state);
            25
        }
        
    }
}