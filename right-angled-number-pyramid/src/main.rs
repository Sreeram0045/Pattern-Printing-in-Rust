fn main()   {
    let height=7;
    for i in 1..=height{
        for j in 1..=i{
            print!("{}",j);
        }
        println!();
    }
}

// output:- 

// 1
// 12
// 123
// 1234
// 12345
// 123456
// 1234567