use super::error::Error;
use super::types::{Atom, Cons, Nil, Symbol, Value};
use std::mem;

pub fn swap(a: &mut Value, b: &mut Value) {
    mem::swap(a, b);
}

pub fn swap_with_car(a: &mut Value, b: &mut Value) -> Result<(), Error> {
    if let Value::Cons(Cons(car, _cdr)) = b {
        mem::swap(a, car);
        Ok(())
    } else {
        Err(Error::SwapWithCarAtom)
    }
}

pub fn swap_with_cdr(a: &mut Value, b: &mut Value) -> Result<(), Error> {
    if let Value::Cons(Cons(_car, cdr)) = b {
        mem::swap(a, cdr);
        Ok(())
    } else {
        Err(Error::SwapWithCdrAtom)
    }
}

pub fn is_nil(v: &Value) -> bool {
    matches!(v, Value::Atom(Atom::Nil))
}

pub fn is_atom(v: &Value) -> bool {
    matches!(v, Value::Atom(_))
}

pub fn eq(lhs: &Atom, rhs: &Atom) -> bool {
    match (lhs, rhs) {
        (Atom::Nil, Atom::Nil) => Nil == Nil,
        (Atom::Symbol(lhs), Atom::Symbol(rhs)) => lhs == rhs,
        _ => false,
    }
}

pub fn assign(dest: &mut Atom, src: Atom) {
    *dest = src;
}

pub fn push(a: &mut Value, b: &mut Value) {
    let car = mem::replace(a, Value::Atom(Atom::Nil));
    let cdr = mem::replace(b, Value::Atom(Atom::Nil));
    *b = Value::Cons(Cons(Box::new(car), Box::new(cdr)));
}

pub fn pop(a: &mut Value, b: &mut Value) -> Result<(), Error> {
    if is_nil(a) {
        if let Value::Cons(Cons(lhs, rhs)) =
            mem::replace(b, Value::Atom(Atom::Nil))
        {
            *a = *lhs;
            *b = *rhs;
            Ok(())
        } else {
            Err(Error::PopFromAtom)
        }
    } else {
        Err(Error::PopIntoNonNil)
    }
}
