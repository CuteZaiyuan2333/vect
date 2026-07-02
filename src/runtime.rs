use crate::isa::{
    calculations::{
        Calculation,
        NumberCalculation,
    }, 
    isa::{
        Instructions,
        Items,
    }, numbers::Types,
};

pub struct Vectvm{
    mainvec: Vec<Items>,
    dropvec: Vec<Items>,
    tempvec: Vec<Items>,
}impl Vectvm{
    pub fn init() -> Self{
        Self{
            mainvec: Vec::new(),
            dropvec: Vec::new(),
            tempvec: Vec::new(),
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
                    self.mainvec.push(item);
                }
            }
        }
    }

    pub fn execute(self: &mut Self){
        
    }
    
    pub fn evaluate(self: &mut Self) -> Items{
        //检查是否为空
        let check = self.mainvec.pop();
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
                            Items::Element(instruction) => {
                                Items::Element(instruction)
                            }
                            //不具备实际意义，只是为了满足match的要求
                            Items::Empty =>{
                                Items::Empty
                            }
                            _ => {
                                return Items::Error;
                            }
                        };
                        //开始匹配指令
                        match op{
                            Items::Element(instruction) =>{
                                return self.dec(instruction);
                            }
                            Items::Empty =>{
                                return Items::Empty;
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
                return self.mux(calculation);
            }
            Instructions::Jump(_op) =>{
                todo!("暂未实现")
            }
            Instructions::Push =>{
                let temp = self.mainvec.pop().unwrap();
                self.dropvec.push(temp.clone());
                self.tempvec.push(temp);
                return Items::Empty;
            }
        }
    }

    pub fn mux(self: &mut Self, calculation: Calculation) -> Items{
        let mut items: [Items; 2] = [Items::Error, Items::Error];
        for i in 0..1{
            let check = self.mainvec.pop();
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
            Calculation::NumberCal(op) =>{
                self.alu(value1, value2, op)
            }
            Calculation::BoolCal(_) =>{
                todo!()
            }
            Calculation::Compare(_) =>{
                todo!()
            }
        };
        return result;
    }
    pub fn alu(self: &mut Self, value1: Types, value2: Types, op: NumberCalculation) -> Items{
        match op {
            NumberCalculation::Add =>{
                Items::Number(value1.add(value2))
            }
            NumberCalculation::Min =>{
                todo!("暂未实现")
            }
            NumberCalculation::Mul =>{
                todo!("暂未实现")
            }
            NumberCalculation::Div =>{
                todo!("暂未实现")
            }
            NumberCalculation::Qtt =>{
                todo!("暂未实现")
            }
            NumberCalculation::Rmd =>{
                todo!("暂未实现")
            }
        }
    }
}

