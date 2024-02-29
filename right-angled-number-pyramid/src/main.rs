fn main()   {
    let height=7;
    for i in 1..=height{
        for j in 1..=i{
            print!("{} ",j);
        }
        println!();
    }
}

// output:- 

// 1 
// 1 2 
// 1 2 3 
// 1 2 3 4 
// 1 2 3 4 5 
// 1 2 3 4 5 6 
// 1 2 3 4 5 6 7 