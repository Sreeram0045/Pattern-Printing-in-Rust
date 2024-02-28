fn main()   {
    let height=5;
    for i in 0..height{
        for _ in 0..i{
            print!(" ");
        }
        for j in 0..(2*height-2*i-1){
            print!("*");
        }
        println!();
    }
}

// output:-

// *********
//  *******
//   *****
//    ***
//     *