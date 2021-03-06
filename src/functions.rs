use super::error::Error;
use super::types::{Atom, Cons, Value};
use std::mem::{swap, take};

pub fn swap_with_car(a: &mut Value, b: &mut Value) -> Result<(), Error> {
    if let Value::Cons(Cons(car, _cdr)) = b {
        swap(a, car);
        Ok(())
    } else {
        Err(Error::SwapWithCarAtom)
    }
}

pub fn swap_with_cdr(a: &mut Value, b: &mut Value) -> Result<(), Error> {
    if let Value::Cons(Cons(_car, cdr)) = b {
        swap(a, cdr);
        Ok(())
    } else {
        Err(Error::SwapWithCdrAtom)
    }
}

pub fn eq(lhs: &Atom, rhs: &Atom) -> bool {
    match (lhs, rhs) {
        (Atom::Nil, Atom::Nil) => true,
        (Atom::Symbol(lhs), Atom::Symbol(rhs)) => lhs == rhs,
        _ => false,
    }
}

pub fn assign(dest: &mut Atom, src: Atom) {
    *dest = src;
}

pub fn push(a: &mut Value, b: &mut Value) {
    let car = take(a);
    let cdr = take(b);
    *b = Value::Cons(Cons(Box::new(car), Box::new(cdr)));
}

pub fn pop(a: &mut Value, b: &mut Value) -> Result<(), Error> {
    if a.is_nil() {
        if let Value::Cons(Cons(lhs, rhs)) = take(b) {
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

pub fn free(v: &mut Value) {
    *v = Value::Atom(Atom::Nil);
}

pub fn copy(src: &Value, dest: &mut Value) -> Result<(), Error> {
    if dest.is_nil() {
        *dest = src.clone();
        Ok(())
    } else {
        Err(Error::CopyIntoNonNil)
    }
}

pub fn equal(lhs: &Value, rhs: &Value) -> bool {
    match (lhs, rhs) {
        (
            Value::Cons(Cons(lhs_car, lhs_cdr)),
            Value::Cons(Cons(rhs_car, rhs_cdr)),
        ) => equal(lhs_car, rhs_car) && equal(lhs_cdr, rhs_cdr),
        (Value::Atom(lhs), Value::Atom(rhs)) => eq(lhs, rhs),
        _ => false,
    }
}
