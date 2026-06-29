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
                                match instruction{
                                    //加法逻辑
                                    Instructions::Add =>{
                                        let pop = [self.scriptvec.pop(), self.scriptvec.pop()];
                                        let mut value: i32 = 0;
                                        for check in pop{
                                            //检查是否有值
                                            match check {
                                                None =>{
                                                    return Items::Error;
                                                }
                                                Some(_) =>{
                                                    let number = check.unwrap();
                                                    //检查值是否合法
                                                    match number {
                                                        //是数字则直接计算
                                                        Items::Number(number) =>{
                                                            value += number;
                                                        }
                                                        //是递归则展开
                                                        Items::Recursion(recursion) =>{
                                                            let mut subvm = Vectvm::init();
                                                            subvm.push(*recursion);
                                                            subvm.rewind();
                                                            let check = subvm.evaluate();
                                                            //检查展开后的递归是否是数字，如果是则进行计算，否则报错
                                                            match check {
                                                                Items::Number(result) =>{
                                                                    value += result;
                                                                }
                                                                _ =>{
                                                                    return Items::Error;
                                                                }
                                                            }
                                                        }
                                                        //如果不是递归或数字则报错
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
                                    Instructions::Larger =>{
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