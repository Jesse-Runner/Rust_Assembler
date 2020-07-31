#![warn(clippy::all)]
extern crate byteorder;

use types::*;
use types::Val::*;
use self::Instr::*;
use types::Unop::*;
use types::Binop::*;
use self::Inst_or_label::*;
use std::env;
use std::str::FromStr;
use std::fs::File;
use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use byteorder::{ByteOrder,BigEndian,WriteBytesExt};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;


 trait from_String where Self: Sized{
     fn from_String(s: &str) -> Result<Self,()>;
 }

pub trait ToBinary {
    fn to_binary(&self) -> Vec<u8>;
}


//Unary Operations
//u ::= neg

#[derive(Debug,Clone)]
pub enum Unop {
    Neg, //Boolean negation
}

impl from_String for Unop{
    fn from_String(s: &str) -> Result<Unop,()>{
        match s{
            "neg" => Ok(Unop::Neg),
            _ => Err(())
        }
    }
}
//Binary Operations
//b ::= + | * | - | / | < | ==

#[derive(Debug,Clone)]
pub enum Binop {
    Add, //i32 addition
    Mul, //i32 multiplication
    Sub, //i32 subtraction
    Div, //i32 division (raises an error on divide by zero)
    Lt,  //Returns true if one i32 is less than another, otherwise false
    Eq,  //Returns true if one i32 is equal another, otherwise false
}

// A method to convert binop strings into their Binop enum type
 impl from_String for Binop{
     fn from_String(s: &str) -> Result<Binop,()>{
        match s{
            "+" => Ok(Binop::Add), // returns addition case
            "-" => Ok(Binop::Sub), // returns subtraction case
            "*" => Ok(Binop::Mul), // returns mult case 
            "/" => Ok(Binop::Div), // returns div case
            "<" => Ok(Binop::Lt), // returns less than case
            "==" => Ok(Binop::Eq), // returns equal to case
            _ => Err(())
        }
     }
 }


//Value types that may appear in GrumpyVM programs:
#[derive(Debug,Clone,PartialEq)]
pub enum Val {
    Vunit,
    Vi32(i32),      //32-bit signed integers
    Vbool(bool),    //Booleans
    Vloc(u32),      //Stack or instruction locations
    Vundef,         //The undefined value
    
}

// Method to convert values as strings into their Val enum type
impl from_String for Val{
    fn from_String(s: &str) -> Result<Val,()>{
        match s{
             s if s.parse::<i32>().is_ok() =>{
                 let y = s.parse::<i32>().unwrap();
                 Ok(Val::Vi32(y))
             }

            s if s.parse::<u32>().is_ok() =>{
                let y = s.parse::<u32>().unwrap();
                 Ok(Val::Vloc(y))
            }
            "tt" => Ok(Val::Vunit),
            "true" => {
                Ok(Val::Vbool(true))
            }
            "false" => {
                Ok(Val::Vbool(false))
            }
            "undef" => Ok(Val::Vundef),
            _ => {
                println!("{}","Stuck in val");
                Err(())
            }
        }
    }

}
// enum to hold the the instruction type
#[derive(Debug,Clone)]
pub enum Instr {
    Push(Val),     //Push(v): Push value v onto the stack      // Push Label onto the stack
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
    Halt           //Halt the machine
 }

 // method to convert a string into its instruction enum type
impl from_String for Instr {
    fn from_String(s: &str) -> Result<Instr,()>{

        if  s.contains(" ") == true{
            let stringbuffer: Vec<&str> = s.split(' ').collect();
            let y = stringbuffer[0];
            let z = stringbuffer[1];

            match y {
                 "push" => return Ok(Instr::Push(Val::from_String(z).unwrap())),
                 "peek" => return Ok(Instr::Peek(z.parse::<u32>().unwrap())),
                 "unary" => return Ok(Instr::Unary(Unop::from_String(z).unwrap())), 
                 "binary" => return Ok(Instr::Binary(Binop::from_String(z).unwrap())),
                 "var" => return Ok(Instr::Var(z.parse::<u32>().unwrap())),
                 "store" => return Ok(Instr::Store(z.parse::<u32>().unwrap())),
                "setframe" => {
                    let numeric = z.parse::<u32>().unwrap();
                    return Ok(Instr::SetFrame(numeric));
                }
                _=> {
                    println!("{}", "Rip");
                    Err(())
                }
            }
        }
        else{
             match s {
                 "pop" => return Ok(Instr::Pop),
                 "swap" => return Ok(Instr::Swap),
                 "alloc" => return Ok(Instr::Alloc),
                 "set" => return Ok(Instr::Set),
                 "get" => return Ok(Instr::Get),
                 "call" => return Ok(Instr::Call),
                 "ret" => return Ok(Instr::Ret),
                 "branch" => return Ok(Instr::Branch),
                 "halt" => return Ok(Instr::Halt),
                   _=> Err(())
            }
         }
    }
}

// enum type to differentiate between an instruction, a label, or a label push
#[derive(Debug,Clone)]
enum Inst_or_label{
    Instruc_hold(Instr),
    Label_hold(String),
    Label_push(String)
}

// method which does the heavy lifting
// it deciphers whether a type Inst_or_label breaks down into
// an instruc_hold, Label_hold, Label_push and then calls the associated enum methods
impl from_String for Inst_or_label {
    fn from_String(s: &str) -> Result<Inst_or_label,()>{
        if s.contains(" "){ 
            let stringbuffer: Vec<&str> = s.split(' ').collect();
            let y = stringbuffer[0];
            let z = stringbuffer[1];
            if y == "push"{
                if z.starts_with("L") || z.starts_with("_"){
                    return Ok(Inst_or_label::Label_push(z.to_string()));
                }
                else {
                    return Ok(Inst_or_label::Instruc_hold(Instr::from_String(s).unwrap()));
                }     
             }
             else{
                return Ok(Inst_or_label::Instruc_hold(Instr::from_String(s).unwrap()));
             }
        }
        else if s.starts_with("L") || s.starts_with("_") {
                return Ok(Inst_or_label::Label_hold(s.to_string()));
        }
        else {
            return Ok(Inst_or_label::Instruc_hold(Instr::from_String(s).unwrap()));
        }
    }
}


// a function to read in the file given by command line argument
// Returns a vector of strings where each index is a line from the file read
fn file_reader(file: String) -> Vec<String>{

    let mut main_stack = Vec::new();

    let f = File::open(file).unwrap();
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let line_push = line.clone();
        main_stack.push(line_push);
    }
    return main_stack;
}

// My two pass algorithm
// Stores instructions or labels into vector vec on first pass
// and pushes labels into the appropriate map
// On the second pass, we cross reference the label push 
// name in our map and resolve the label location 
fn map_Vector(vec: Vec<String>) -> Vec<Instr>  {
    let mut holder: Vec<Inst_or_label> = Vec::new();
    let mut holder1: Vec<Instr> = Vec::new();
    let mut mymap:HashMap<String,u32> = HashMap::new();
    let mut pc = 0;
   for x in vec {
        let y = Inst_or_label::from_String(&x).unwrap();
          match &y {
              Label_hold(b) => {
                  let mut myvar = b.clone();
                  myvar.truncate(myvar.len()-1);
                  mymap.insert(myvar.to_string(),pc);
              }
              _ => pc = pc + 1
          }
        let zzz = y.clone();
        holder.push(zzz);
    }
    for x in holder{
        match x{
            Label_push(a) => {
                if mymap.contains_key(&a){
                    holder1.push(Instr::Push(Val::Vloc(*mymap.get(&a).unwrap())));
                }
            }
            Instruc_hold(c) => holder1.push(c),
            _ => ()
        }
    }
    return holder1;
}

    // function to convert our i32 values into big endian 
    impl ToBinary for i32 {
        fn to_binary(&self) -> Vec<u8>{
            let mut bytes = vec![];
            bytes.write_i32::<BigEndian>(*self).unwrap();
            return bytes;
        }
    }
    // function to convert our u32 values into big endian
    impl ToBinary for u32 {
        fn to_binary(&self) -> Vec<u8>{
            let mut bytes = vec![];
            bytes.write_u32::<BigEndian>(*self).unwrap();
            return bytes;
        }
    }
    // function to convert a unop instruction into byte form
    impl ToBinary for Unop {
        fn to_binary(&self) -> Vec<u8>{
            let mut bytes: Vec<u8> = vec![];
            match self {
                Unop::Neg => {
                    bytes.append(&mut vec![0b00000000]);
                    return bytes;
                }
            }
        }
    }
    // function to convert varying binop instructions into byte code
    impl ToBinary for Binop {
        fn to_binary(&self) -> Vec<u8>{
            let mut bytes = vec![];
            match self{
                Binop::Add => {
                    bytes.append(&mut vec![0b00000000]);
                    return bytes;
                }
                Binop::Sub => {
                    bytes.append(&mut vec![0b00000010]);
                    return bytes;
                }
                Binop::Mul =>{
                    bytes.append(&mut vec![0b00000001]);
                    return bytes;
                }
                Binop::Div => {
                    bytes.append(&mut vec![0b00000011]);
                    return bytes;
                }
                Binop::Lt =>{
                    bytes.append(&mut vec![0b00000100]);
                    return bytes;
                }
                Binop::Eq =>{
                    bytes.append(&mut vec![0b00000101]);
                    return bytes;
                }
            }

        }
    }
    // function to convert various val types into byte code
     impl ToBinary for Val {
         fn to_binary(&self) -> Vec<u8>{
            let mut bytes = vec![];
            match self {
                Val::Vi32(x) => {
                    bytes.append(&mut vec![0b00000001]);
                    bytes.append(&mut <i32 as ToBinary>::to_binary(x));
                    return bytes;
                }
                Val::Vbool(x) => {
                    match x {
                        true => {
                            bytes.append(&mut vec![0b00000010]);
                            return bytes;
                        }
                        false => {
                            bytes.append(&mut vec![0b00000011]);
                            return bytes;
                        }
                    }
                }
                Val::Vunit => vec![0b00000000],
                Val::Vloc(x) => {
                    bytes.append(&mut vec![0b00000100]);
                    bytes.append(&mut <u32 as ToBinary>::to_binary(x));
                    return bytes;
                }
                Val::Vundef => {
                    bytes.append(&mut vec![0b00000101]);
                    return bytes;
                }
            }
         }
     }

    // functions to convert various instruction types into binary
     impl ToBinary for Instr {
         fn to_binary(&self) -> Vec<u8>{
            let mut bytes = vec![];
            match self{
                Push(x) => {
                    bytes.append(&mut vec![0b00000000]);
                    bytes.append(&mut <Val as ToBinary>::to_binary(x));
                    return bytes; 
                }
                Pop => {
                    bytes.append(&mut vec![0b00000001]);
                    return bytes;
                }
                Peek(x) =>{
                    bytes.append(&mut vec![0b00000010]);
                    bytes.append(&mut <u32 as ToBinary>::to_binary(x));
                    return bytes;

                }
                Unary(x) =>{
                    bytes.append(&mut vec![0b00000011]);
                    bytes.append(&mut <Unop as ToBinary>::to_binary(x));
                    return bytes;
                }
                Binary(x) =>{
                    bytes.append(&mut vec![0b00000100]);
                    bytes.append(&mut <Binop as ToBinary>::to_binary(x));
                    return bytes;
                }
                Swap => {
                    bytes.append(&mut vec![0b00000101]);
                    return bytes;
                }
                Alloc => {
                    bytes.append(&mut vec![0b00000110]);
                    return bytes;
                }
                Set => {
                    bytes.append(&mut vec![0b00000111]);
                    return bytes;
                }
                Get => {
                    bytes.append(&mut vec![0b00001000]);
                    return bytes;
                }
                Var(x) => {
                    bytes.append(&mut vec![0b00001001]);
                    bytes.append(&mut <u32 as ToBinary>::to_binary(x));
                    return bytes;
                }
                Store(x) => {
                    bytes.append(&mut vec![0b00001010]);
                    bytes.append(&mut <u32 as ToBinary>::to_binary(x));
                    return bytes;
                }
                SetFrame(x) => {
                    bytes.append(&mut vec![0b00001011]);
                    bytes.append(&mut <u32 as ToBinary>::to_binary(x));
                    return bytes;
                }
                Call => {
                    bytes.append(&mut vec![0b00001100]);
                    return bytes;
                }
                Ret => {
                    bytes.append(&mut vec![0b00001101]);
                    return bytes;
                }
                Branch => {
                    bytes.append(&mut vec![0b00001110]);
                    return bytes;
                }
                Halt => {
                    bytes.append(&mut vec![0b00001111]);
                    return bytes;
                }
            }
         }
     }


     // main function which calls all of our methods above
     // and ultimately writes our binary vector into our file
    fn main() {
    
    let args: Vec<String> = env::args().collect(); // collects command line argument of filename
    let query = &args[1].to_string(); // query holds the command line argument
    let storage = file_reader(query.to_string()); // populates storage vector with contents of file given
    let mapped_Storage = map_Vector(storage); // maps storage vector into our abstract syntax

    let mut binaryfile = query.clone();
     binaryfile.truncate(query.len() - 1);
     binaryfile.push_str("o");

    let mut bytes: Vec<u8> = Vec::new();
    let total_instr = mapped_Storage.len() as u32;
    bytes.append(&mut <u32 as ToBinary>::to_binary(&total_instr));
     for x in mapped_Storage{
        bytes.append(&mut Instr::to_binary(&x));
    }
    fs::write(binaryfile, &bytes).expect("Failed to write");
    std::process::exit(0);
}
