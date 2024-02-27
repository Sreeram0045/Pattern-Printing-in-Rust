fn main()   {
    let height=7;
    for i in 1..=height{
        let num=format!("{} ",i).repeat(i);
        println!("{}",num);
    }
}

// output:-

// 1 
// 2 2 
// 3 3 3 
// 4 4 4 4 
// 5 5 5 5 5 
// 6 6 6 6 6 6 
// 7 7 7 7 7 7 7 