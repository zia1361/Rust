pub mod table
{
    pub fn print_table(number: i32)
    {
        for item in 1..11
        {
            println!("{} * {} = {}",number,item,number*item);
        }
    }
}
pub mod odd_lib
{
    pub fn print_odd(limit1:i32,limit2:i32)
    {
        let mut number1 = limit1;
        let mut number2 = limit2;
        for item in limit1..limit2+1
        {
            if number1%2 == 1
            {
                println!("{}",number1);
            }
            number1 +=1;
            
        }
    }
}
pub mod even_lib
{
    pub fn print_even(limit1:i32,limit2:i32)
    {
        let mut number1 = limit1;
        let mut number2 = limit2;
        for item in limit1..limit2+1
        {
            if number1%2 == 0
            {
                println!("{}",number1);
            }
            number1 +=1;
            
        }
    }
}
pub mod pow_lib
{
    pub fn power(number:i32, power: i32)->i32
    {
        let mut output = number;
        for item in 1..power
        {
            output *=number;
        }
        output
    }
}