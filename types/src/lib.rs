#![warn(clippy::all)]
use self::Binop::*;
use self::Instr::*;
use self::Unop::*;
use self::Val::*;

pub trait ToBinary {
    fn to_binary(&self) -> Vec<u8>;
}

type Address = usize;
type Err = ();

/*
 * il ::= i      // An il is either an instruction
 *      | l:     // or a label "l" followed by a colon ":" as in "Lmain:"
*/

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    //Value types that may appear in GrumpyVM programs:
    Vunit,       //The unit value
    Vi32(i32),   //32-bit signed integers
    Vbool(bool), //Booleans
    Vloc(u32),   //Stack or instruction locations
    Vundef,      //The undefined value

    //Value types that are used internally by the language implementation, and may not appear in GrumpyVM programs:
    Vsize(i32),     //Metadata for heap objects that span multiple values
    Vaddr(Address), //Pointers to heap locations
}

#[derive(Debug, Clone)]
pub enum Instr {
    Push(Val),     //Push(v): Push value v onto the stack
    Pop,           //Pop a value from the stack, discarding it
    Peek(u32),     //Peek(i): Push onto the stack the ith value from the top
    Unary(Unop),   //Unary(u): Apply u to the top value on the stack
    Binary(Binop), //Binary(b): Apply b to the top two values on the stack, replacing them with the result
    Swap,          //Swap the top two values
    Alloc,         //Allocate an array on the heap
    Set,           //Write to a heap-allocated array
    Get,           //Read from a heap-allocated array
    Var(u32),      //Var(i): Get the value at stack position fp+i
    Store(u32),    //Store(i): Store a value at stack position fp+i
    SetFrame(u32), //SetFrame(i): Set fp = s.stack.len() - i
    Call,          //Function call
    Ret,           //Function return
    Branch,        //Conditional jump
    Halt,          //Halt the machine
}

#[derive(Debug, Clone)]
pub enum Unop {
    Neg, //Boolean negation
}

#[derive(Debug, Clone)]
pub enum Binop {
    Add, //i32 addition
    Mul, //i32 multiplication
    Sub, //i32 subtraction
    Div, //i32 division (raises an error on divide by zero)
    Lt,  //Returns true if one i32 is less than another, otherwise false
    Eq,  //Returns true if one i32 is equal another, otherwise false
}

// Put all your test cases in this module
#[cfg(test)]
mod tests {
    use super::*;
    use super::Binop::*;
    use super::Instr::*;
    use super::Unop::*;
    use super::Val::*;

    // Replace the following test case with one that tests your code
    #[test]
    fn test_1() {
        assert_eq!(1 + 2, 3);
    }
}
