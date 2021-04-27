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

impl Default for Atom {
    /// Returns the empty value `Nil`
    fn default() -> Self {
        Self::Nil
    }
}

#[derive(Clone)]
pub enum Value {
    Atom(Atom),
    Cons(Cons),
}

impl Default for Value {
    /// Returns the empty value `Nil`
    fn default() -> Self {
        Self::Atom(Atom::default())
    }
}

impl Value {
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Atom(Atom::Nil))
    }

    pub fn is_atom(&self) -> bool {
        matches!(self, Value::Atom(_))
    }
}
