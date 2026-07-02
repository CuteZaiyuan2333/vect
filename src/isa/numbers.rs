#[derive(Clone)]
pub enum Types{
    Bool(bool),
    I32(i32),
    F32(f32),
    I64(i64),
    F64(f64),
    Error,
}
//number calculations
impl Types{
    pub fn add(self, input: Types) -> Types{
        match (self, input) {
            (Types::I32(value1), Types::I32(value2)) =>{
                return Types::I32(value1 + value2);
            }
            (Types::I64(value1), Types::I64(value2)) =>{
                return Types::I64(value1 + value2);
            }
            (Types::F32(value1), Types::F32(value2)) =>{
                return Types::F32(value1 + value2);
            }
            (Types::F64(value1), Types::F64(value2)) =>{
                return Types::F64(value1 + value2);
            }

            (Types::I32(value1), Types::I64(value2)) =>{
                return Types::I64(value1 as i64 + value2);
            }
            (Types::I64(value1), Types::I32(value2)) =>{
                return Types::I64(value1 + value2 as i64);
            }
            (Types::F32(value1), Types::F64(value2)) =>{
                return Types::F64(value1 as f64 + value2);
            }
            (Types::F64(value1), Types::F32(value2)) =>{
                return Types::F64(value1 + value2 as f64);
            }

            (Types::I32(value1), Types::F32(value2)) =>{
                return Types::F32(value1 as f32 + value2);
            }
            (Types::I64(value1), Types::F64(value2)) =>{
                return Types::F64(value1 as f64 + value2);
            }
            (Types::F32(value1), Types::I32(value2)) =>{
                return Types::F32(value1 + value2 as f32);
            }
            (Types::F64(value1), Types::I64(value2)) =>{
                return Types::F64(value1 + value2 as f64);
            }

            (Types::I32(value1), Types::F64(value2)) =>{
                return Types::F64(value1 as f64 + value2);
            }
            (Types::I64(value1), Types::F32(value2)) =>{
                return Types::F64(value1 as f64 + value2 as f64);
            }
            (Types::F32(value1), Types::I64(value2)) =>{
                return Types::F64(value1 as f64 + value2 as f64);
            }
            (Types::F64(value1), Types::I32(value2)) =>{
                return Types::F64(value1 as f64 + value2 as f64);
            }

            _ =>{
                return Types::Error;
            }
        }
    }
    pub fn sub(self, input: Types) -> Types{
        match (self, input) {
            (Types::I32(value1), Types::I32(value2)) =>{
                return Types::I32(value1 - value2);
            }
            (Types::I64(value1), Types::I64(value2)) =>{
                return Types::I64(value1 - value2);
            }
            (Types::F32(value1), Types::F32(value2)) =>{
                return Types::F32(value1 - value2);
            }
            (Types::F64(value1), Types::F64(value2)) =>{
                return Types::F64(value1 - value2);
            }

            (Types::I32(value1), Types::I64(value2)) =>{
                return Types::I64(value1 as i64 - value2);
            }
            (Types::I64(value1), Types::I32(value2)) =>{
                return Types::I64(value1 - value2 as i64);
            }
            (Types::F32(value1), Types::F64(value2)) =>{
                return Types::F64(value1 as f64 - value2);
            }
            (Types::F64(value1), Types::F32(value2)) =>{
                return Types::F64(value1 - value2 as f64);
            }

            (Types::I32(value1), Types::F32(value2)) =>{
                return Types::F32(value1 as f32 - value2);
            }
            (Types::I64(value1), Types::F64(value2)) =>{
                return Types::F64(value1 as f64 - value2);
            }
            (Types::F32(value1), Types::I32(value2)) =>{
                return Types::F32(value1 - value2 as f32);
            }
            (Types::F64(value1), Types::I64(value2)) =>{
                return Types::F64(value1 - value2 as f64);
            }

            (Types::I32(value1), Types::F64(value2)) =>{
                return Types::F64(value1 as f64 - value2);
            }
            (Types::I64(value1), Types::F32(value2)) =>{
                return Types::F64(value1 as f64 - value2 as f64);
            }
            (Types::F32(value1), Types::I64(value2)) =>{
                return Types::F64(value1 as f64 - value2 as f64);
            }
            (Types::F64(value1), Types::I32(value2)) =>{
                return Types::F64(value1 as f64 - value2 as f64);
            }

            _ =>{
                return Types::Error;
            }
        }
    }
    pub fn mul(self, input: Types) -> Types{
        match (self, input) {
            (Types::I32(value1), Types::I32(value2)) =>{
                return Types::I32(value1 * value2);
            }
            (Types::I64(value1), Types::I64(value2)) =>{
                return Types::I64(value1 * value2);
            }
            (Types::F32(value1), Types::F32(value2)) =>{
                return Types::F32(value1 * value2);
            }
            (Types::F64(value1), Types::F64(value2)) =>{
                return Types::F64(value1 * value2);
            }

            (Types::I32(value1), Types::I64(value2)) =>{
                return Types::I64(value1 as i64 * value2);
            }
            (Types::I64(value1), Types::I32(value2)) =>{
                return Types::I64(value1 * value2 as i64);
            }
            (Types::F32(value1), Types::F64(value2)) =>{
                return Types::F64(value1 as f64 * value2);
            }
            (Types::F64(value1), Types::F32(value2)) =>{
                return Types::F64(value1 * value2 as f64);
            }

            (Types::I32(value1), Types::F32(value2)) =>{
                return Types::F32(value1 as f32 * value2);
            }
            (Types::I64(value1), Types::F64(value2)) =>{
                return Types::F64(value1 as f64 * value2);
            }
            (Types::F32(value1), Types::I32(value2)) =>{
                return Types::F32(value1 * value2 as f32);
            }
            (Types::F64(value1), Types::I64(value2)) =>{
                return Types::F64(value1 * value2 as f64);
            }

            (Types::I32(value1), Types::F64(value2)) =>{
                return Types::F64(value1 as f64 * value2);
            }
            (Types::I64(value1), Types::F32(value2)) =>{
                return Types::F64(value1 as f64 * value2 as f64);
            }
            (Types::F32(value1), Types::I64(value2)) =>{
                return Types::F64(value1 as f64 * value2 as f64);
            }
            (Types::F64(value1), Types::I32(value2)) =>{
                return Types::F64(value1 as f64 * value2 as f64);
            }

            _ =>{
                return Types::Error;
            }
        }
    }
    pub fn div(self, input: Types) -> Types{
        match (self, input) {
            (Types::I32(value1), Types::I32(value2)) =>{
                return Types::F32(value1 as f32 / value2 as f32);
            }
            (Types::I64(value1), Types::I64(value2)) =>{
                return Types::F64(value1 as f64 / value2 as f64);
            }
            (Types::F32(value1), Types::F32(value2)) =>{
                return Types::F32(value1 / value2);
            }
            (Types::F64(value1), Types::F64(value2)) =>{
                return Types::F64(value1 / value2);
            }

            (Types::I32(value1), Types::I64(value2)) =>{
                return Types::F64(value1 as f64 / value2 as f64);
            }
            (Types::I64(value1), Types::I32(value2)) =>{
                return Types::F64(value1 as f64 - value2 as f64);
            }
            (Types::F32(value1), Types::F64(value2)) =>{
                return Types::F64(value1 as f64 / value2);
            }
            (Types::F64(value1), Types::F32(value2)) =>{
                return Types::F64(value1 / value2 as f64);
            }

            (Types::I32(value1), Types::F32(value2)) =>{
                return Types::F32(value1 as f32 / value2);
            }
            (Types::I64(value1), Types::F64(value2)) =>{
                return Types::F64(value1 as f64 / value2);
            }
            (Types::F32(value1), Types::I32(value2)) =>{
                return Types::F32(value1 / value2 as f32);
            }
            (Types::F64(value1), Types::I64(value2)) =>{
                return Types::F64(value1 / value2 as f64);
            }

            (Types::I32(value1), Types::F64(value2)) =>{
                return Types::F64(value1 as f64 / value2);
            }
            (Types::I64(value1), Types::F32(value2)) =>{
                return Types::F64(value1 as f64 / value2 as f64);
            }
            (Types::F32(value1), Types::I64(value2)) =>{
                return Types::F64(value1 as f64 / value2 as f64);
            }
            (Types::F64(value1), Types::I32(value2)) =>{
                return Types::F64(value1 as f64 / value2 as f64);
            }

            _ =>{
                return Types::Error;
            }
        }
    }
    pub fn qtt(self, input: Types) -> Types{
        match (self, input) {
            (Types::I32(value1), Types::I32(value2)) =>{
                return Types::I32(value1 / value2);
            }
            (Types::I64(value1), Types::I64(value2)) =>{
                return Types::I64(value1 / value2);
            }
            (Types::F32(value1), Types::F32(value2)) =>{
                return Types::I32(value1.div_euclid(value2) as i32);
            }
            (Types::F64(value1), Types::F64(value2)) =>{
                return Types::I64(value1.div_euclid(value2) as i64);
            }

            (Types::I32(value1), Types::I64(value2)) =>{
                return Types::I64(value1 as i64 / value2);
            }
            (Types::I64(value1), Types::I32(value2)) =>{
                return Types::I64(value1 /  value2 as i64);
            }
            (Types::F32(value1), Types::F64(value2)) =>{
                return Types::I64((value1 as f64).div_euclid(value2) as i64);
            }
            (Types::F64(value1), Types::F32(value2)) =>{
                return Types::I64(value1.div_euclid(value2 as f64) as i64);
            }

            (Types::I32(value1), Types::F32(value2)) =>{
                return Types::I32((value1 as f32).div_euclid(value2) as i32);
            }
            (Types::I64(value1), Types::F64(value2)) =>{
                return Types::I64((value1 as f64).div_euclid(value2) as i64);
            }
            (Types::F32(value1), Types::I32(value2)) =>{
                return Types::I32(value1.div_euclid(value2 as f32) as i32);
            }
            (Types::F64(value1), Types::I64(value2)) =>{
                return Types::I64(value1.div_euclid(value2 as f64) as i64);
            }

            (Types::I32(value1), Types::F64(value2)) =>{
                return Types::I64((value1 as f64).div_euclid(value2) as i64);
            }
            (Types::I64(value1), Types::F32(value2)) =>{
                return Types::I64((value1 as f64).div_euclid(value2 as f64) as i64);
            }
            (Types::F32(value1), Types::I64(value2)) =>{
                return Types::I64((value1 as f64).div_euclid(value2 as f64) as i64);
            }
            (Types::F64(value1), Types::I32(value2)) =>{
                return Types::I64((value1 as f64).div_euclid(value2 as f64) as i64);
            }

            _ =>{
                return Types::Error;
            }
        }
    }
    pub fn rmd(self, input: Types) -> Types{
        match (self, input) {
            (Types::I32(value1), Types::I32(value2)) =>{
                return Types::I32(value1 % value2);
            }
            (Types::I64(value1), Types::I64(value2)) =>{
                return Types::I64(value1 % value2);
            }
            (Types::F32(value1), Types::F32(value2)) =>{
                return Types::F32(value1.rem_euclid(value2));
            }
            (Types::F64(value1), Types::F64(value2)) =>{
                return Types::F64(value1.rem_euclid(value2));
            }

            (Types::I32(value1), Types::I64(value2)) =>{
                return Types::I64(value1 as i64 % value2);
            }
            (Types::I64(value1), Types::I32(value2)) =>{
                return Types::I64(value1 % value2 as i64);
            }
            (Types::F32(value1), Types::F64(value2)) =>{
                return Types::F64((value1 as f64).rem_euclid(value2));
            }
            (Types::F64(value1), Types::F32(value2)) =>{
                return Types::F64(value1.rem_euclid(value2 as f64));
            }

            (Types::I32(value1), Types::F32(value2)) =>{
                return Types::F32((value1 as f32).rem_euclid(value2));
            }
            (Types::I64(value1), Types::F64(value2)) =>{
                return Types::F64((value1 as f64).rem_euclid(value2));
            }
            (Types::F32(value1), Types::I32(value2)) =>{
                return Types::F32(value1.rem_euclid(value2 as f32));
            }
            (Types::F64(value1), Types::I64(value2)) =>{
                return Types::F64(value1.rem_euclid(value2 as f64));
            }

            (Types::I32(value1), Types::F64(value2)) =>{
                return Types::F64((value1 as f64).rem_euclid(value2));
            }
            (Types::I64(value1), Types::F32(value2)) =>{
                return Types::F64((value1 as f64).rem_euclid(value2 as f64));
            }
            (Types::F32(value1), Types::I64(value2)) =>{
                return Types::F64((value1 as f64).rem_euclid(value2 as f64));
            }
            (Types::F64(value1), Types::I32(value2)) =>{
                return Types::F64((value1 as f64).rem_euclid(value2 as f64));
            }

            _ =>{
                return Types::Error;
            }
        }
    }
}

//boolean calculations
impl Types{
    pub fn and(self, input: Types) -> Types{
        match self {
            Types::Bool(value1) =>{
                match input {
                    Types::Bool(value2) =>{
                        return Types::Bool(value1 & value2);
                    }
                    _ => Types::Error
                }
            }
            _ => Types::Error
        }
    }
    pub fn or(self, input: Types) -> Types{
        match self {
            Types::Bool(value1) =>{
                match input {
                    Types::Bool(value2) =>{
                        return Types::Bool(value1 || value2);
                    }
                    _ => Types::Error
                }
            }
            _ => Types::Error
        }
    }
    pub fn not(self) -> Types{
        match self {
            Types::Bool(value1) =>{
                return Types::Bool(!value1)
            }
            _ => Types::Error
        }
    }
}

//compare
impl Types{
    pub fn lgr(self, input: Types) -> Types{
        match (self, input) {
            (Types::I32(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 > value2);
            }
            (Types::I64(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 > value2);
            }
            (Types::F32(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 > value2);
            }
            (Types::F64(value1), Types::F64(value2)) =>{
                return Types::Bool(value1 > value2);
            }

            (Types::I32(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 as i64 > value2);
            }
            (Types::I64(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 > value2 as i64);
            }
            (Types::F32(value1), Types::F64(value2)) =>{
                return Types::Bool(value1 as f64 > value2);
            }
            (Types::F64(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 > value2 as f64);
            }

            (Types::I32(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 as f32 > value2);
            }
            (Types::I64(value1), Types::F64(value2)) =>{
                return Types::Bool(value1 as f64 > value2);
            }
            (Types::F32(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 > value2 as f32);
            }
            (Types::F64(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 > value2 as f64);
            }

            (Types::I32(value1), Types::F64(value2)) =>{
                return Types::Bool(value1 as f64 > value2);
            }
            (Types::I64(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 as f64 > value2 as f64);
            }
            (Types::F32(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 as f64 > value2 as f64);
            }
            (Types::F64(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 as f64 > value2 as f64);
            }

            _ =>{
                return Types::Error;
            }
        }
    }
    pub fn les(self, input: Types) -> Types{
        match (self, input) {
            (Types::I32(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 < value2);
            }
            (Types::I64(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 < value2);
            }
            (Types::F32(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 < value2);
            }
            (Types::F64(value1), Types::F64(value2)) =>{
                return Types::Bool(value1 < value2);
            }

            (Types::I32(value1), Types::I64(value2)) =>{
                return Types::Bool((value1 as i64) < value2);
            }
            (Types::I64(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 < value2 as i64);
            }
            (Types::F32(value1), Types::F64(value2)) =>{
                return Types::Bool((value1 as f64) < value2);
            }
            (Types::F64(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 < value2 as f64);
            }

            (Types::I32(value1), Types::F32(value2)) =>{
                return Types::Bool((value1 as f32) < value2);
            }
            (Types::I64(value1), Types::F64(value2)) =>{
                return Types::Bool((value1 as f64) < value2);
            }
            (Types::F32(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 < value2 as f32);
            }
            (Types::F64(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 < value2 as f64);
            }

            (Types::I32(value1), Types::F64(value2)) =>{
                return Types::Bool((value1 as f64) < value2);
            }
            (Types::I64(value1), Types::F32(value2)) =>{
                return Types::Bool((value1 as f64) < value2 as f64);
            }
            (Types::F32(value1), Types::I64(value2)) =>{
                return Types::Bool((value1 as f64) < value2 as f64);
            }
            (Types::F64(value1), Types::I32(value2)) =>{
                return Types::Bool((value1 as f64) < value2 as f64);
            }

            _ =>{
                return Types::Error;
            }
        }
    }
    pub fn eql(self, input: Types) -> Types{
        match (self, input) {
            (Types::I32(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 == value2);
            }
            (Types::I64(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 == value2);
            }
            (Types::F32(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 == value2);
            }
            (Types::F64(value1), Types::F64(value2)) =>{
                return Types::Bool(value1 == value2);
            }

            (Types::I32(value1), Types::I64(value2)) =>{
                return Types::Bool((value1 as i64) == value2);
            }
            (Types::I64(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 == value2 as i64);
            }
            (Types::F32(value1), Types::F64(value2)) =>{
                return Types::Bool((value1 as f64) == value2);
            }
            (Types::F64(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 == value2 as f64);
            }

            (Types::I32(value1), Types::F32(value2)) =>{
                return Types::Bool((value1 as f32) == value2);
            }
            (Types::I64(value1), Types::F64(value2)) =>{
                return Types::Bool((value1 as f64) == value2);
            }
            (Types::F32(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 == value2 as f32);
            }
            (Types::F64(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 == value2 as f64);
            }

            (Types::I32(value1), Types::F64(value2)) =>{
                return Types::Bool((value1 as f64) == value2);
            }
            (Types::I64(value1), Types::F32(value2)) =>{
                return Types::Bool((value1 as f64) == value2 as f64);
            }
            (Types::F32(value1), Types::I64(value2)) =>{
                return Types::Bool((value1 as f64) == value2 as f64);
            }
            (Types::F64(value1), Types::I32(value2)) =>{
                return Types::Bool((value1 as f64) == value2 as f64);
            }

            _ =>{
                return Types::Error;
            }
        }
    }
    pub fn eqlgr(self, input: Types) -> Types{
        match (self, input) {
            (Types::I32(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 >= value2);
            }
            (Types::I64(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 >= value2);
            }
            (Types::F32(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 >= value2);
            }
            (Types::F64(value1), Types::F64(value2)) =>{
                return Types::Bool(value1 >= value2);
            }

            (Types::I32(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 as i64 >= value2);
            }
            (Types::I64(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 >= value2 as i64);
            }
            (Types::F32(value1), Types::F64(value2)) =>{
                return Types::Bool(value1 as f64 >= value2);
            }
            (Types::F64(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 >= value2 as f64);
            }

            (Types::I32(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 as f32 >= value2);
            }
            (Types::I64(value1), Types::F64(value2)) =>{
                return Types::Bool(value1 as f64 >= value2);
            }
            (Types::F32(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 >= value2 as f32);
            }
            (Types::F64(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 >= value2 as f64);
            }

            (Types::I32(value1), Types::F64(value2)) =>{
                return Types::Bool(value1 as f64 >= value2);
            }
            (Types::I64(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 as f64 >= value2 as f64);
            }
            (Types::F32(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 as f64 >= value2 as f64);
            }
            (Types::F64(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 as f64 >= value2 as f64);
            }

            _ =>{
                return Types::Error;
            }
        }
    }
    pub fn eqles(self, input: Types) -> Types{
        match (self, input) {
            (Types::I32(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 <= value2);
            }
            (Types::I64(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 <= value2);
            }
            (Types::F32(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 <= value2);
            }
            (Types::F64(value1), Types::F64(value2)) =>{
                return Types::Bool(value1 <= value2);
            }

            (Types::I32(value1), Types::I64(value2)) =>{
                return Types::Bool((value1 as i64) <= value2);
            }
            (Types::I64(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 <= value2 as i64);
            }
            (Types::F32(value1), Types::F64(value2)) =>{
                return Types::Bool((value1 as f64) <= value2);
            }
            (Types::F64(value1), Types::F32(value2)) =>{
                return Types::Bool(value1 <= value2 as f64);
            }

            (Types::I32(value1), Types::F32(value2)) =>{
                return Types::Bool((value1 as f32) <= value2);
            }
            (Types::I64(value1), Types::F64(value2)) =>{
                return Types::Bool((value1 as f64) <= value2);
            }
            (Types::F32(value1), Types::I32(value2)) =>{
                return Types::Bool(value1 <= value2 as f32);
            }
            (Types::F64(value1), Types::I64(value2)) =>{
                return Types::Bool(value1 <= value2 as f64);
            }

            (Types::I32(value1), Types::F64(value2)) =>{
                return Types::Bool((value1 as f64) <= value2);
            }
            (Types::I64(value1), Types::F32(value2)) =>{
                return Types::Bool((value1 as f64) <= value2 as f64);
            }
            (Types::F32(value1), Types::I64(value2)) =>{
                return Types::Bool((value1 as f64) <= value2 as f64);
            }
            (Types::F64(value1), Types::I32(value2)) =>{
                return Types::Bool((value1 as f64) <= value2 as f64);
            }

            _ =>{
                return Types::Error;
            }
        }
    }
}