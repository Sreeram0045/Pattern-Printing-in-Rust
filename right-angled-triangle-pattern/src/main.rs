
fn main()   {
    let height=5;
    for i in 0..height{
        let num="* ".repeat(i+1);
        println!("{}",num);
    }
}

// repeat function adds the string "* " i+1 to the variable num, which is then printed later

// output:-

// * 
// * * 
// * * * 
// * * * * 
// * * * * * 