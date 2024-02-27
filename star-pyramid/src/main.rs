fn main()   {
    let height=5;
    for i in 0..height{
        for _ in (0..height-i-1).rev(){
            print!(" ");
        }
        for _ in 0..(i*2 + 1){
            print!("*");
        }
        println!();
    }
}


// output:-

//     *
//    ***
//   *****
//  *******
// *********