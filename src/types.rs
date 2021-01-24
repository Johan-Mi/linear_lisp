#[derive(PartialEq, Eq)]
pub struct Nil;

#[derive(Clone, PartialEq, Eq)]
pub struct Symbol(String);

#[derive(Clone)]
pub struct Cons(pub Box<Value>, pub Box<Value>);

#[derive(Clone)]
pub enum Atom {
    Nil,
    Symbol(Symbol),
}

#[derive(Clone)]
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
