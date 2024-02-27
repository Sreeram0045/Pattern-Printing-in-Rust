
fn main()   {
    let height=7;
    for i in (0..height).rev(){
        for j in (1..=i+1).rev(){
            print!("{} ",j);
        }
        println!();
    }
}

// output:-

// 7 6 5 4 3 2 1 
// 6 5 4 3 2 1 
// 5 4 3 2 1 
// 4 3 2 1 
// 3 2 1 
// 2 1 
// 1 