extern crate Maths;
use Maths::table as table_lib;
use Maths::odd_lib as odd_lib;
use Maths::even_lib as even_lib;
use Maths::pow_lib as pow_lib;
fn main() {
    println!("printing table");
   table_lib::print_table(2);
   println!("printing odd numbers");
   odd_lib::print_odd(1,10);
   println!("printing even numbers");
   even_lib::print_even(1,10);
   println!("printing power");
   println!("{}",pow_lib::power(2,4));



}
