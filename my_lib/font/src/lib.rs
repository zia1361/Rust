
// pub mod font
// {
//   pub  mod upper
//     {
//        pub fn to_upper(input:String)->String
//        {
           
//        }
        
//     }
//      pub  mod lower
//     {
        
     
//          pub fn lower_case(input:String)->String{
//              let  capital = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
//              let  small = String::from("abcdefghijklmnopqrstuvwxyz");
//             let mut counter = 0;
//             let mut counter2 = 1;
//             let mut convert_input = String::new();
            
//             loop
//             {
                
//                 if input[counter..counter2] == capital[counter..counter2] || input[counter..counter2] == small[counter..counter2]
//                  {
//                            convert_input +=&small[counter..counter2];
//                  }
//                 counter +=1;
//                 counter2+=1;
//                 if counter==input.len()
//                 {
//                     break;
//                 }
//             }
//             convert_input
//         }
//     }
  
// }
//   pub mod read
//     {
//         pub mod string
//         {
//             use std::io;
//             pub fn read_string()->String
//          {
//             let mut input = String::new();
//             io::stdin().read_line(& mut input).expect("failed to read");
//             input
//          }
//         }
//     }
