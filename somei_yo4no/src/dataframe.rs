use std::mem;

#[derive(Debug)]
pub struct DataFrame {
    pub base: Vec<u8>,
    pub values: Vec<Value>
}

#[derive(Debug)]
pub struct Value {
    pub value_type: ValueType,
    pub data: Vec<u8>
}

#[derive(Debug)]
pub enum ValueType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    U8Vec,
    String,
    Bool
}

impl Value {
    pub fn new(value_type: ValueType, data: Vec<u8>) -> Value {
        Value {
            value_type,
            data
        }
    }
    pub fn from_u8(data: u8) -> Value {
        Value {
            value_type: ValueType::U8,
            data: vec![data]
        }
    }
    pub fn from_u16(data: u16) -> Value {
        unsafe {
            Value {
                value_type: ValueType::U16,
                data: mem::transmute::<u16, [u8; 2]>(data).to_vec()
            }
        }
    }
    pub fn from_u32(data: u32) -> Value {
        unsafe {
            Value {
                value_type: ValueType::U32,
                data: mem::transmute::<u32, [u8; 4]>(data).to_vec()
            }
        }
    }
    pub fn from_u64(data: u64) -> Value {
        unsafe {
            Value {
                value_type: ValueType::U64,
                data: mem::transmute::<u64, [u8; 8]>(data).to_vec()
            }
        }
    }
    pub fn from_i8(data: i8) -> Value {
        Value {
            value_type: ValueType::I8,
            data: vec![data as u8]
        }
    }
    pub fn from_i16(data: i16) -> Value {
        unsafe {
            Value {
                value_type: ValueType::I16,
                data: mem::transmute::<i16, [u8; 2]>(data).to_vec()
            }
        }
    }
    pub fn from_i32(data: i32) -> Value {
        unsafe {
            Value {
                value_type: ValueType::I32,
                data: mem::transmute::<i32, [u8; 4]>(data).to_vec()
            }
        }
    }
    pub fn from_i64(data: i64) -> Value {
        unsafe {
            Value {
                value_type: ValueType::I64,
                data: mem::transmute::<i64, [u8; 8]>(data).to_vec()
            }
        }
    }
    pub fn from_f32(data: f32) -> Value {
        unsafe {
            Value {
                value_type: ValueType::F32,
                data: mem::transmute::<f32, [u8; 4]>(data).to_vec()
            }
        }
    }
    pub fn from_f64(data: f64) -> Value {
        unsafe {
            Value {
                value_type: ValueType::F64,
                data: mem::transmute::<f64, [u8; 8]>(data).to_vec()
            }
        }
    }
    pub fn from_str(data: String) -> Value {
        Value {
            value_type: ValueType::String,
            data: data.as_bytes().to_vec()
        }
    }
    pub fn from_u8_vec(data: Vec<u8>) -> Value {
        Value {
            value_type: ValueType::U8Vec,
            data: data
        }
    }
    pub fn as_u8(&self) -> Result<u8, ()> {
        if let ValueType::U8 = self.value_type {
            Ok(self.data[0] as u8)
        }else{
            Err(())
        }
    }
    pub fn as_u16(&self) -> Result<u16, ()> {
        if let ValueType::U16 = self.value_type {
            unsafe {
                let mut arr = [0; 2];
                arr.copy_from_slice(&self.data);
                Ok(mem::transmute::<[u8; 2], u16>(arr))
            }
        }else{
            Err(())
        }
    }
    pub fn as_u32(&self) -> Result<u32, ()> {
        if let ValueType::U32 = self.value_type {
            unsafe {
                let mut arr = [0; 4];
                arr.copy_from_slice(&self.data);
                Ok(mem::transmute::<[u8; 4], u32>(arr))
            }
        }else{
            Err(())
        }
    }
    pub fn as_u64(&self) -> Result<u64, ()> {
        if let ValueType::U64 = self.value_type {
            unsafe {
                let mut arr = [0; 8];
                arr.copy_from_slice(&self.data);
                Ok(mem::transmute::<[u8; 8], u64>(arr))
            }
        }else{
            Err(())
        }
    }
    pub fn as_i8(&self) -> Result<i8, ()> {
        if let ValueType::I8 = self.value_type {
            Ok(self.data[0] as i8)
        }else{
            Err(())
        }
    }
    pub fn as_i16(&self) -> Result<i16, ()> {
        if let ValueType::I16 = self.value_type {
            unsafe {
                let mut arr = [0; 2];
                arr.copy_from_slice(&self.data);
                Ok(mem::transmute::<[u8; 2], i16>(arr))
            }
        }else{
            Err(())
        }
    }
    pub fn as_i32(&self) -> Result<i32, ()> {
        if let ValueType::I32 = self.value_type {
            unsafe {
                let mut arr = [0; 4];
                arr.copy_from_slice(&self.data);
                Ok(mem::transmute::<[u8; 4], i32>(arr))
            }
        }else{
            Err(())
        }
    }
    pub fn as_i64(&self) -> Result<i64, ()> {
        if let ValueType::I64 = self.value_type {
            unsafe {
                let mut arr = [0; 8];
                arr.copy_from_slice(&self.data);
                Ok(mem::transmute::<[u8; 8], i64>(arr))
            }
        }else{
            Err(())
        }
    }
    pub fn as_f32(&self) -> Result<f32, ()> {
        if let ValueType::F32 = self.value_type {
            unsafe {
                let mut arr = [0; 4];
                arr.copy_from_slice(&self.data);
                Ok(mem::transmute::<[u8; 4], f32>(arr))
            }
        }else{
            Err(())
        }
    }
    pub fn as_f64(&self) -> Result<f64, ()> {
        if let ValueType::F64 = self.value_type {
            unsafe {
                let mut arr = [0; 8];
                arr.copy_from_slice(&self.data);
                Ok(mem::transmute::<[u8; 8], f64>(arr))
            }
        }else{
            Err(())
        }
    }
    pub fn as_str(&self) -> Result<String, ()> {
        if let ValueType::String = self.value_type {
            Ok(String::from_utf8_lossy(&self.data).into_owned())
        }else{
            Err(())
        }
    }
    pub fn as_slice(&self) -> Result<&[u8], ()> {
        if let ValueType::U8Vec = self.value_type {
            Ok(&self.data)
        }else{
            Err(())
        }
    }
}

impl DataFrame {
    pub fn new() -> DataFrame {
        DataFrame {
            base: Vec::new(),
            values: Vec::new()
        }
    }
    pub fn from(buf: &[u8]) -> Result<DataFrame,()> {
        use ValueType::*;
        let mut df = DataFrame::new();
        df.base=buf.clone().to_vec();
        let buf = buf.to_vec();
        let mut i = 0;
        while (buf[i] & 0b10000000) == 128 {
            let opcode = buf[i] & 0b00001111;
            i+=1;
            let mut len: usize = (buf[i] & 0b01111111) as usize;
            i+=1;
            if len == 126 {
                unsafe {
                    let mut arr=[0; 2];
                    arr.copy_from_slice(&buf[i..(i+2)]);
                    len = mem::transmute::<[u8; 2], u16>(arr) as usize;
                }
                i+=2;
            } else if len == 127 {
                unsafe {
                    let mut arr=[0; 8];
                    arr.copy_from_slice(&buf[i..(i+8)]);
                    len = mem::transmute::<[u8; 8], u64>(arr) as usize;
                }
                i+=8;
            }
            let data: Vec<u8> = buf[i..(len+i)].to_vec();
            match opcode {
                0 => {
                    match len {
                        1 => {
                            df.values.push(
                                Value::new(
                                    U8,
                                    (&data[0..1]).to_vec()
                                )
                            );
                        },
                        2 => {
                            df.values.push(
                                Value::new(
                                    U16,
                                    (&data[0..2]).to_vec()
                                )
                            );
                        },
                        4 => {
                            df.values.push(
                                Value::new(
                                    U32,
                                    (&data[0..4]).to_vec()
                                )
                            );
                        },
                        8 => {
                            df.values.push(
                                Value::new(
                                    U64,
                                    (&data[0..8]).to_vec()
                                )
                            );
                        },
                        _ => {
                            return Result::Err(());
                        }
                    }
                },
                1 => {
                    match len {
                        1 => {
                            df.values.push(
                                Value::new(
                                    I8,
                                    (&data[0..1]).to_vec()
                                )
                            );
                        },
                        2 => {
                            df.values.push(
                                Value::new(
                                    I16,
                                    (&data[0..2]).to_vec()
                                )
                            );
                        },
                        4 => {
                            df.values.push(
                                Value::new(
                                    I32,
                                    (&data[0..4]).to_vec()
                                )
                            );
                        },
                        8 => {
                            df.values.push(
                                Value::new(
                                    I64,
                                    (&data[0..8]).to_vec()
                                )
                            );
                        },
                        _ => {
                            return Result::Err(());
                        }
                    }
                },
                2 => {
                    match len {
                        4 => {
                            df.values.push(
                                Value::new(
                                    F32,
                                    (&data[0..4]).to_vec()
                                )
                            );
                        },
                        8 => {
                            df.values.push(
                                Value::new(
                                    F64,
                                    (&data[0..8]).to_vec()
                                )
                            );
                        },
                        _ => {
                            return Result::Err(());
                        }
                    }
                },
                3 => {
                    df.values.push(
                        Value::new(
                            U8Vec,
                            data.to_vec()
                        )
                    );
                },
                4 => {
                    df.values.push(
                        Value::new(
                            String,
                            data.to_vec()
                        )
                    );
                },
                5 => {
                    df.values.push(
                        Value::new(
                            Bool,
                            (&data[0..1]).to_vec()
                        )
                    );
                },
                _ => {
                    return Result::Err(());
                }
            }
        }
        Result::Ok(df)
    }
    pub fn as_slice(&self) -> &[u8] {
        return self.base.as_slice();
    }
    pub fn as_bytes(&mut self) -> &[u8] {
        use ValueType::*;
        let mut i = 0;
        for v in &self.values {
            let opcode = match v.value_type {
                U8 | U16 | U32 | U64 => 0,
                I8 | I16 | I32 | I64 => 1,
                F32 | F64 => 2,
                U8Vec => 3,
                String => 4,
                Bool => 5
            };
            let len: usize = v.data.len();
            self.base[i] = 0b10000000 + (opcode & 0b00001111);
            i+=1;
            match len {
                0 ... 125 => {
                    self.base[i] = len as u8;
                },
                126 ... 65535 => unsafe {
                    self.base[i] = 126;
                    i+=1;
                    let arr = mem::transmute::<u16, [u8; 2]>(len as u16);
                    for j in 0..2 {
                        self.base[i] = arr[j];
                        i+=1;
                    }
                },
                _ => unsafe {
                    self.base[i] = 127;
                    i+=1;
                    let arr = mem::transmute::<u64, [u8; 8]>(len as u64);
                    for j in 0..8 {
                        self.base[i] = arr[j];
                        i+=1;
                    }
                }
            }
            self.base.append(&mut v.data.clone());
        }
        return self.base.as_slice();
    }
    pub fn push(&mut self, v: Value) {
        self.values.push(v);
    }
    pub fn get(&self, i: usize) -> &Value {
        return &self.values[i]
    }
}
