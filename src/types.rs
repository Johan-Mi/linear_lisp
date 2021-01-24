#[derive(PartialEq, Eq)]
pub struct Nil;

#[derive(PartialEq, Eq)]
pub struct Symbol(String);

pub struct Cons(pub Box<Value>, pub Box<Value>);

pub enum Atom {
    Nil,
    Symbol(Symbol),
}

pub enum Value {
    Atom(Atom),
    Cons(Cons),
}

impl Value {
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Atom(Atom::Nil))
    }

    pub fn is_atom(&self) -> bool {
        matches!(self, Value::Atom(_))
    }
}
