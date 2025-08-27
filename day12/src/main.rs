use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("Hello, world!");
    let mut hello = String::from("नमस्ते");
    hello.push_str("in hindi");
    hello.push('.');
    println!("{hello}");
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1=String::from("Helllloo");
    // let h=s1[0];
    
        // for c in hello.as_bytes(){
        //     println!("{c}");
        // }
    for e in hello.graphemes(true).collect::<Vec<&str>>(){
        println!("E={e}");
    }
}
