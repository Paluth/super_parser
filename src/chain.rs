use std::any::Any;

pub enum Operation<'c> {
    Trim,
    TrimL,
    TrimR,
    EqualString(String),
    NotEqualString(String),
    Empty,
    ToI32,
    ToF64,
    ToU8,
    ToByte,
    Or(Vec<Chain<'c>>),
    Store(Box<&'c mut Any>),
    Opt(Chain<'c>),
}

pub struct Chain<'c> {
    pub olist: Vec<Operation<'c>>,
}

impl<'c> Chain<'c> {
    fn new() -> Self {
        Chain { olist: vec![] }
    }

    pub fn trim(mut self) -> Self {
        self.olist.push(Operation::Trim);
        self
    }

    pub fn triml(mut self) -> Self {
        self.olist.push(Operation::TrimL);
        self
    }

    pub fn trimr(mut self) -> Self {
        self.olist.push(Operation::TrimR);
        self
    }

    pub fn eqs(mut self, s: String) -> Self {
        self.olist.push(Operation::EqualString(s));
        self
    }

    pub fn eq_str(mut self, s: &str) -> Self {
        self.olist.push(Operation::EqualString(s.to_string()));
        self
    }

    pub fn neqs(mut self, s: String) -> Self {
        self.olist.push(Operation::NotEqualString(s));
        self
    }

    pub fn neq_str(mut self, s: &str) -> Self {
        self.olist.push(Operation::NotEqualString(s.to_string()));
        self
    }

    pub fn empty(mut self) -> Self {
        self.olist.push(Operation::Empty);
        self
    }

    pub fn or(mut self, ops: Vec<Chain<'c>>) -> Self {
        self.olist.push(Operation::Or(ops));
        self
    }

    pub fn to_byte(mut self) -> Self {
        self.olist.push(Operation::ToByte);
        self
    }

    pub fn to_u8(mut self) -> Self {
        self.olist.push(Operation::ToU8);
        self
    }

    pub fn to_i32(mut self) -> Self {
        self.olist.push(Operation::ToI32);
        self
    }

    pub fn to_f64(mut self) -> Self {
        self.olist.push(Operation::ToF64);
        self
    }

    pub fn store<T>(mut self, field: &'c mut T) -> Self
        where T: Any
    {
        self.olist.push(Operation::Store(Box::new(field)));
        self
    }

    pub fn opt(mut self, chain : Chain<'c>) -> Self {
        self.olist.push(Operation::Opt(chain));
        self
    }
}

pub fn to_byte<'c>() -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::ToByte);
    c
}

pub fn to_u8<'c>() -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::ToU8);
    c
}

pub fn to_i32<'c>() -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::ToI32);
    c
}

pub fn to_f64<'c>() -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::ToF64);
    c
}

pub fn empty<'c>() -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::Empty);
    c
}

pub fn trim<'c>() -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::Trim);
    c
}

pub fn triml<'c>() -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::TrimL);
    c
}

pub fn trimr<'c>() -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::TrimR);
    c
}

pub fn eqs<'c>(s: String) -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::EqualString(s));
    c
}

pub fn eq_str<'c>(s: &str) -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::EqualString(s.to_string()));
    c
}

pub fn neqs<'c>(s: String) -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::NotEqualString(s));
    c
}

pub fn neq_str<'c>(s: &str) -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::NotEqualString(s.to_string()));
    c
}

pub fn or<'c>(ops: Vec<Chain<'c>>) -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::Or(ops));
    c
}

pub fn opt<'c>(chain : Chain<'c>) -> Chain<'c> {
    let mut c = Chain::new();
    c.olist.push(Operation::Opt(chain));
    c
}

pub fn store<'c, 'o, T>(field: &'c mut T) -> Chain<'c>
    where T: Any
{
    let mut c = Chain::new();
    c.olist.push(Operation::Store(Box::new(field)));
    c
}