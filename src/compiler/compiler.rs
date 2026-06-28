// use crate::isa::{
//     Instructions,
//     Items,
// };

// pub struct Compiler{
//     input: String,
//     result: Vec<Items>,
// }impl Compiler{
//     pub fn init() -> Self{
//         Self{
//             input: "".to_string(),
//             result: Vec::new(),
//         }
//     }
//     pub fn vect_asm_load(self: &mut Self, string: String){
//         self.input = string;
//         //未来应该实现读取文本的功能
//     }
//     pub fn vect_asm_lexer(self: &mut Self){
//         let script: &str = &self.input;
//         let mut chars = script.chars().peekable();
//         while let Some(&mychar) = chars.peek(){
//             match mychar{
//                 '(' =>{
                    
//                 }
//                 ')' =>{
                    
//                 }
//                 ','| ' '| '\t'| '\r'| '\n'| ';'  =>{
                    
//                 }
//                 _ =>{
                    
//                 }
//             }
//         }
//     }
// }