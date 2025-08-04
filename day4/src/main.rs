// Data Types in Rust

fn main() {
    println!("Data Types");

    let a=254u8;
    let b =1_00_00_000;
    let c=b'A';
    let d =65.12345689101112131411516177181920;//f64 is default for decimal valeus
    let e='z';
    let f="Kaushik Samadder";
    let emoji='😂';

    //* Tuples is a general way of grouping together.
    let info=("Kaushik",21,true,41000);
    let (name,age,indian,bank_balance)=info;


    //*  Arrays

    let arrays =[1,2,3,4,5];
    let arrays1:[f64;5] =[1.0,2.0,3.0,4.0,5.6];
    let arrays2 =[10;5];


    println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",a,b,c,d,e,f,emoji,info.0,name,age,indian,bank_balance);
    
    // Integer OverFlow
    // If I store a number bigger than the data type size, then it causes integer overflow

}
