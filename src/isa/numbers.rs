#[derive(Clone)]
pub enum Types{
    Bool(bool),
    I32(i32),
    F32(f32),
    I64(i64),
    F64(f64),
    Error,
}impl Types{
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
}