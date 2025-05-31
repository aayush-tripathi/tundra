// bytecode/value.rs
use std::{cell::RefCell, fmt, rc::Rc};
use crate::{bytecode::chunk::Chunk};
///Defines the Value Types Tundra can understand
#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Bool(bool),
    None,
    String(String),
    Int(i64),
    Float(f64),
    Char(char),
     Array(Rc<RefCell<Vec<Value>>>),
    // A Rust‐side builtin function:
    NativeFunction(fn(&[Value]) -> Value, usize /*arity*/),
    // A user‐defined function 
    Function(Rc<RefCell<FunctionObject>>),

}
#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub value: ValueType,
}
/* ───────── Encoding helpers ───────── */

pub const TAG_BITS : u64 = 0b111;
pub const TAG_PTR  : u64 = 0b000;
pub const TAG_INT  : u64 = 0b001;
pub const TAG_BOOL : u64 = 0b010;
pub const TAG_NONE : u64 = 0b011;
pub const TAG_CHAR : u64 = 0b100;   



impl Value{
    ///A Bool Value
    pub fn boolean(val: bool) -> Value {
		Value { 
			value: ValueType::Bool(val)
		}
	}
    ///A Float64 Value
	pub fn float(val: f64) -> Value {
		Value {
			value: ValueType::Float(val)
		}
	}
    ///An Int64 Value
    pub fn int(val:i64) -> Value {
        Value {
            value: ValueType::Int(val)
        }
    }
    ///A Character
    pub fn char(val: char) -> Value {
        Value {
            value: ValueType::Char(val)
        }
    }
    ///A String
    pub fn string(val: String) -> Value {
        Value {
            value: ValueType::String(val)
        }
    }
    ///None Type Value
    pub fn none() -> Value {
        Value {
            value: ValueType::None
        }
    }
    ///Native Function
    pub fn native(func: fn(&[Value]) -> Value, arity: usize) -> Self {
        Value { value: ValueType::NativeFunction(func, arity) }
    }
    ///User Defined Function
    pub fn function(fobj: FunctionObject) -> Self {
    Value {
      value: ValueType::Function(Rc::new(RefCell::new(fobj)))
    }
  }
    ///Array of a Value type
     pub fn array(v: Vec<Value>) -> Self {
        Value { value: ValueType::Array(Rc::new(RefCell::new(v))) }
    }
    ///Convert to Bool
	pub fn to_bool(val: Value) -> bool {
		match val.value {
			ValueType::Bool(val) => val,
            ValueType::Int(val)=>val!=0,
            ValueType::Float(val)=>val!=0.0,
            ValueType::None=>false,
			_ => panic!("Do not call Value::to_bool on a non-bool type")
		}
	}
	///Convert to Float64
	pub fn to_float(val: Value) -> f64 {
		match val.value {
			ValueType::Float(val) => val,
            ValueType::Int(val)=>val as f64,
			_ => panic!("Do not call Value::to_float on a non-float type")
		}
	}
    //Convert to String
    pub fn to_string(val: Value) -> String {
        match val.value {
            ValueType::String(string) => string,
            ValueType::Int(val) => val.to_string(),
            ValueType::Char(val) => val.to_string(),
            ValueType::None => "".to_string(),
            _ => panic!("Do not call Value::to_string on a non-string type"),
        }
    }
    ///Convert to Int64
    pub fn to_int(val: &Value) -> i64 {
        match val.value {
            ValueType::Int(val)=>val,
            ValueType::Float(val)=>val as i64,
            _ => panic!("Do not call Value::to_string on a non-string type"),
        }
    }
    /// Convert to Character
    pub fn to_char(val: &Value) -> char {
        match val.value {
            ValueType::Char(val)=>val,
            _ => panic!("Do not call Value::to_string on a non-string type"),
        }
    }
    ///Returns True if val is boolean
    pub fn is_bool(val: &Value) -> bool {
        match val.value {
            ValueType::Bool(_) => true,
            _ => false,
        }
    }
    ///Returns True if val is a string
    pub fn is_string(val: &Value) -> bool {
        match val.value {
            ValueType::String(_) => true,
            _ => false,
        }
    }
    /// loss-less, type-preserving packing into a single register
     pub fn as_i64(&self) -> i64 {
        match &self.value {
            /* POD scalar slots */
            ValueType::Int(i)        => ((*i as u64) << 3 | TAG_INT)          as i64,
            ValueType::Bool(false)   => TAG_BOOL as i64,
            ValueType::Bool(true)    => (1u64 << 3 | TAG_BOOL)               as i64,
            ValueType::None          => TAG_NONE as i64,
            ValueType::Char(c)       => ((*c as u64) << 3 | TAG_CHAR)        as i64,
            ValueType::Float(f)      => f.to_bits()                          as i64,

            /* everything heap-allocated – we just use the raw pointer      */
            ValueType::String(s)     => { let p = s.as_ptr() as u64;      (p | TAG_PTR) as i64 }
            ValueType::Array(rc)     => { let p = Rc::as_ptr(rc) as u64;  (p | TAG_PTR) as i64 }
            ValueType::Function(rc)  => { let p = Rc::as_ptr(rc) as u64;  (p | TAG_PTR) as i64 }
            ValueType::NativeFunction(f, _) =>
                                        { let p = *f as *const () as u64; (p | TAG_PTR) as i64 }
        }
    }

    pub unsafe fn from_i64(raw: i64) -> Self {
        let bits = raw as u64;
        match bits & TAG_BITS {
            TAG_INT  => Self::int((bits >> 3) as i64),
            TAG_BOOL => Self::boolean((bits >> 3) & 1 == 1),
            TAG_NONE => Self::none(),
            TAG_CHAR => {
                let scalar = ((bits >> 3) & 0x001F_FFFF) as u32;
                let ch = std::char::from_u32(scalar).unwrap_or('\u{FFFD}');
                Self::char(ch)

            }
            TAG_PTR  => Self { value: ValueType::from_heap_ptr((bits & !TAG_BITS) as *const ()) },
            _        => panic!("unknown value-tag {:03b}", bits & TAG_BITS),
        }
    }
    

    
}
impl Value {
    ///View Value as Bytes of form (Tag + Payload Value)
    pub fn to_bytes(&self) -> Vec<u8> {
        match &self.value {
            ValueType::Int(v) => {
                let mut b = vec![0x01];            // Value‐type tag for Int
                b.extend(&v.to_le_bytes());         // i64 → 8 bytes
                b
            }
            ValueType::Float(v) => {
                let mut b = vec![0x02];            // Float tag
                b.extend(&v.to_le_bytes());         // f64 → 8 bytes
                b
            }
            ValueType::Bool(v) => {
                vec![0x03, *v as u8]                // Bool tag + 1 byte
            }
            ValueType::None => {
                vec![0x04]                          // None tag, no payload
            }
            ValueType::Char(c) => {
                let mut b = vec![0x05];            // Char tag
                b.extend(&(*c as u32).to_le_bytes()); // char → 4-byte Unicode scalar
                b
            }
            ValueType::String(s) => {
                let mut b = vec![0x06];            // String tag
                let bytes = s.as_bytes();
                b.extend(&(bytes.len() as u32).to_le_bytes()); // length
                b.extend(bytes);                    // raw UTF-8
                b
            }
            _ => panic!("Cannot serialize {:?}", self.value),
        }

    }

}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ValueType::Int(v)    => write!(f, "{}", v),
            ValueType::Float(v)  => write!(f, "{}", v),
            ValueType::Char(v)   => write!(f, "{}", v),
            ValueType::None      => write!(f, "None"),
            ValueType::String(s) => write!(f, "\"{}\"", s),
            ValueType::Bool(v)   => write!(f, "{}", v),
            _ => panic!("Cannot Display {:?}", self.value),
        }
    }
}
///Struct for User-Defined Function
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionObject {
    pub name: String,
    pub arity: usize,
    pub chunk: Rc<RefCell<Chunk>>,
    pub jitted: Option<*const u8>,
}
impl FunctionObject {
    ///Create new user-defined function
    pub fn new(name: String, arity: usize, chunk: Rc<RefCell<Chunk>>) -> Self {
        FunctionObject { name, arity, chunk,jitted:None, }
    }
}
#[repr(u8)]
enum HeapTag { Str = 1, Arr, Fun, Native }

#[repr(C)]
struct HeapHeader { tag: HeapTag }
impl ValueType {
    unsafe fn from_heap_ptr(ptr: *const ()) -> Self {
        let hdr   = &*(ptr as *const HeapHeader);
        match hdr.tag {
            HeapTag::Str   => {
                let rc = Rc::from_raw(ptr as *const String);
                let v  = ValueType::String((*rc).clone());
                Rc::into_raw(rc);                  
                v
            }
            HeapTag::Arr   => {
                let rc = Rc::from_raw(ptr as *const RefCell<Vec<Value>>);
                let v  = ValueType::Array(rc.clone());
                Rc::into_raw(rc);
                v
            }
            HeapTag::Fun   => {
                let rc = Rc::from_raw(ptr as *const RefCell<FunctionObject>);
                let v  = ValueType::Function(rc.clone());
                Rc::into_raw(rc);
                v
            }
            HeapTag::Native => {
                let f   = ptr as *const ();
                ValueType::NativeFunction(std::mem::transmute(f), 0)
            }
        }
    }
}