
fn main() {
    let height = 7;
    for i in (0..height).rev() {
        let stars = "* ".repeat(i + 1);
        println!("{}", stars);
    }
}

// output:-

// * * * * * 
// * * * * 
// * * * 
// * * 
// * 