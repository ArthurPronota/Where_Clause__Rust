use std::fmt::Display ;

pub fn print_all<I>(item: I) 
    where I: IntoIterator,
          I::Item: Display
{
    for x in item {
        println!("{}", x) ;
    }
}
    
fn main() {
    print_all(vec![1,2,3,4,5]); /* Out:
    1
    2
    3
    4
    5    
     */
}
