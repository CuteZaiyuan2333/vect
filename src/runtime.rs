use crate::isa::{
    calculations::Calculation, 
    isa::{
        Instructions,
        Items,
    },
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
        //检查是否为空
        let check = self.scriptvec.pop();
        self.dropvec.push(check.clone().unwrap());
        match check{
            None => {
                return Items::Error;
            }
            Some(_) => {
                //检查是否合法
                let op = check.unwrap();
                match op{
                    Items::Error =>{
                        return Items::Error;
                    }
                    Items::Number(_) =>{
                        return Items::Error;
                    }
                    _ =>{
                        //检查类型是指令还是返回递归的指令
                        let op = match op {
                            //是递归则展开并返回指令
                            Items::Recursion(recursion) =>{
                                let mut subvm = Vectvm::init();
                                subvm.push(*recursion);
                                subvm.rewind();
                                let check = subvm.evaluate();
                                //检查递归展开之后是否是指令
                                match check {
                                    Items::Element(instruction) => Items::Element(instruction),
                                    //展开后不是则报错
                                    _ =>{
                                        return Items::Error;
                                    }
                                }
                            }
                            //是指令则返回指令
                            Items::Element(instruction) => Items::Element(instruction),
                            //不具备实际意义，只是为了满足match的要求
                            _ => {
                                return Items::Error;
                            }
                        };
                        //开始匹配指令
                        match op{
                            Items::Element(instruction) =>{
                                return self.dec(instruction);
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

    pub fn dec(self: &mut Self, instruction: Instructions) -> Items{
        match instruction {
            Instructions::Cal(calculation) =>{
                return self.alu(calculation);
            }
            Instructions::Jump =>{
                todo!("暂未实现")
            }
            Instructions::Larger =>{
                todo!("暂未实现")
            }
        }
    }

    pub fn alu(self: &mut Self, calculation: Calculation) -> Items{
        let mut items: [Items; 2] = [Items::Error, Items::Error];
        for i in 0..1{
            let check = self.scriptvec.pop();
            match check {
                None =>{
                    return Items::Error;
                }
                Some(item) =>{
                    match item {
                        Items::Recursion(recursion) =>{
                            let mut subvm = Vectvm::init();
                            subvm.push(*recursion);
                            subvm.rewind();
                            items[i] = subvm.evaluate();
                        }
                        Items::Number(number) =>{
                            items[i] = Items::Number(number)
                        }
                        _ =>{
                            return Items::Error
                        }
                    }
                }
            }
        }

        let (value1, value2) = (items[0].popnumber(), items[1].popnumber());
        
        let result: Items = match calculation {
            Calculation::Add =>{
                Items::Number(value1.add(value2))
            }
            Calculation::Min =>{
                todo!("暂未实现")
            }
            Calculation::Mul =>{
                todo!("暂未实现")
            }
            Calculation::Div =>{
                todo!("暂未实现")
            }
            Calculation::IntDiv =>{
                todo!("暂未实现")
            }
            Calculation::Rmd =>{
                todo!("暂未实现")
            }
        };
        return result;
    }
}

