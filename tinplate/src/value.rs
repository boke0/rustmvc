use std::mem;

pub enum Value {
    Float(f64),
    Int(i64),
    String(String),
    Null,
    Bytes(Vec<u8>)
}
impl Value {
    pub fn to_vec(&mut self) -> Vec<u8>{
        let mut buf=Vec::new();
        match self {
            Value::Float(v) => {
                buf.push(0);
                unsafe {
                    let mut bytes=mem::transmute::<f64, [u8; 8]>(*v).to_vec();
                    buf.append(&mut bytes);
                }
            },
            Value::Int(v) => {
                buf.push(1);
                unsafe {
                    let mut bytes=mem::transmute::<i64, [u8; 8]>(*v).to_vec();
                    buf.append(&mut bytes);
                }
            },
            Value::String(v) => {
                buf.push(2);
                buf.append(
                    &mut v.as_str()
                        .as_bytes()
                        .to_vec()
                );
            },
            Value::Null => {
                buf.push(3);
            },
            Value::Bytes(v) => {
                buf.push(4);
                buf.append(v);
            }
        }
        buf
    }
    pub fn from(v: &[u8]) -> Value {
        match &v[0] {
            0 => unsafe {
                let mut buf:[u8; 8] = [0; 8];
                for i in 0..8 {
                    buf[i]=v[(i+1)];
                }
                let v_=mem::transmute::<[u8; 8],f64>(buf);
                Value::Float(v_)
            },
            1 => unsafe {
                let mut buf:[u8; 8] = [0; 8];
                for i in 0..8 {
                    buf[i]=v[(i+1)];
                }
                let v_=mem::transmute::<[u8; 8],i64>(buf);
                Value::Int(v_)
            },
            2 => Value::String(String::from_utf8_lossy(v).into_owned()),
            3 => Value::Null,
            4 => Value::Bytes(v.to_vec()),
            _ => Value::Null
        }
    }
}
