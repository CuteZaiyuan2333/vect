use crate::isa::{
    Instructions,
    Items,
};

pub struct Vectvm{
    scriptvec: Vec<Items>,
    dropvec: Vec<Items>,
}impl Vectvm{
    pub fn init() -> Self{
        Self{
            scriptvec: Vec::new(),
            dropvec: Vec::new(),
        }
    }
    
    pub fn push(self: &mut Self, script: Vec<Items>){
        self.dropvec = script;
        self.rewind();
    }
    
    pub fn rewind(self: &mut Self){
        for _ in 0..self.dropvec.len(){
            let check = self.dropvec.pop();
            match check {
                None => {
                    println!("Error")
                }
                Some(_) =>{
                    let item = check.unwrap();
                    self.scriptvec.push(item);
                }
            }
        }
    }
    
    pub fn evaluate(self: &mut Self) -> Items{
        let check = self.scriptvec.pop();
        self.dropvec.push(check.clone().unwrap());
        match check{
            None => {
                return Items::Error;
            }
            Some(_) => {
                let op = check.unwrap();
                match op{
                    Items::Error =>{
                        return Items::Error;
                    }
                    Items::Number(_) =>{
                        return Items::Error;
                    }
                    _ =>{
                        let op = match op {
                            Items::Recursion(recursion) =>{
                                let mut subvm = Vectvm::init();
                                subvm.push(*recursion);
                                let check = subvm.evaluate();
                                match check {
                                    Items::Element(instruction) => Items::Element(instruction),
                                    _ =>{
                                        return Items::Error;
                                    }
                                }
                            }
                            Items::Element(instruction) => Items::Element(instruction),
                            _ => {
                                return Items::Error;
                            }
                        };
                        match op{
                            Items::Element(instruction) =>{
                                match instruction{
                                    Instructions::Add =>{
                                        let pop = [self.scriptvec.pop(), self.scriptvec.pop()];
                                        let mut value: i32 = 0;
                                        for check in pop{
                                            match check {
                                                None =>{
                                                    return Items::Error;
                                                }
                                                Some(_) =>{
                                                    let number = check.unwrap();
                                                    match number {
                                                        Items::Number(number) =>{
                                                            value += number;
                                                        }
                                                        Items::Recursion(recursion) =>{
                                                            let mut subvm = Vectvm::init();
                                                            subvm.push(*recursion);
                                                            subvm.rewind();
                                                            let check = subvm.evaluate();
                                                            match check {
                                                                Items::Number(result) =>{
                                                                    value += result;
                                                                }
                                                                _ =>{
                                                                    return Items::Error;
                                                                }
                                                            }
                                                        }
                                                        _ =>{
                                                            return Items::Error;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        return Items::Number(value);
                                    }
                                    Instructions::Jump =>{
                                        todo!("暂未实现")
                                    }
                                    Instructions::Lgr =>{
                                        todo!("暂未实现")
                                    }
                                }
                            }
                            _ =>{
                                return Items::Error;
                            }
                        }
                    }
                }
            }
        };
    }
}