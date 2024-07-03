use std::{fmt, ops::Add};


#[derive(Debug,Clone)]
pub enum ValueType{
    Bool(bool),
    None,
    String(String),
    Int(i64),
    Float(f64),
    Char(char),
}
#[derive(Debug,Clone)]
pub struct Value{
   pub value:ValueType,
}

impl Value{
    pub fn boolean(val: bool) -> Value {
		Value { 
			value: ValueType::Bool(val)
		}
	}

	pub fn float(val: f64) -> Value {
		Value {
			value: ValueType::Float(val)
		}
	}
    pub fn int(val:i64) -> Value {
        Value {
            value: ValueType::Int(val)
        }
    }
    
    pub fn char(val: char) -> Value {
        Value {
            value: ValueType::Char(val)
        }
    }

    pub fn string(val: String) -> Value {
        Value {
            value: ValueType::String(val)
        }
    }

    pub fn none() -> Value {
        Value {
            value: ValueType::None
        }
    }

	pub fn to_bool(val: Value) -> bool {
		match val.value {
			ValueType::Bool(val) => val,
            ValueType::Int(val)=>val!=0,
            ValueType::Float(val)=>val!=0.0,
            ValueType::None=>false,
			_ => panic!("Do not call Value::to_bool on a non-bool type")
		}
	}
		
	pub fn to_float(val: Value) -> f64 {
		match val.value {
			ValueType::Float(val) => val,
            ValueType::Int(val)=>val as f64,
			_ => panic!("Do not call Value::to_float on a non-float type")
		}
	}

    pub fn to_string(val: Value) -> String {
        match val.value {
            ValueType::String(string) => string,
            ValueType::Int(val) => val.to_string(),
            ValueType::Char(val) => val.to_string(),
            ValueType::None => "".to_string(),
            _ => panic!("Do not call Value::to_string on a non-string type"),
        }
    }

    pub fn to_int(val: &Value) -> i64 {
        match val.value {
            ValueType::Int(val)=>val,
            ValueType::Float(val)=>val as i64,
            _ => panic!("Do not call Value::to_string on a non-string type"),
        }
    }
   
    pub fn to_char(val: &Value) -> char {
        match val.value {
            ValueType::Char(val)=>val,
            _ => panic!("Do not call Value::to_string on a non-string type"),
        }
    }

    pub fn is_bool(val: &Value) -> bool {
        match val.value {
            ValueType::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_string(val: &Value) -> bool {
        match val.value {
            ValueType::String(_) => true,
            _ => false,
        }
    }

    

    
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.value.clone();
        match val {
            ValueType::Int(v) => write!(f, "{}", v),
            ValueType::Float(v) => write!(f, "{}", v),
            ValueType::Char(v) => write!(f, "{}", v),
            ValueType::None => write!(f, "None"),
            ValueType::String(s) => write!(f, "\"{}\"", s),
            ValueType::Bool(v) => write!(f, "{}", v),
        }
    }
}
