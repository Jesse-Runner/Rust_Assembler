setframe 0
push Lmain
call
halt
Lmain:
push undef // var 2 
push undef // var 3
push 1
push undef
alloc
store 2
var 2 // this keeps the location of the array on the top of the stack (vbase)
push 0 // this pushes an index of the array (vidx)
push _L6 // value to push onto the heap (vnew)
set    // sets vbase[vidx] = vnew
var 2 // retrieves the location of the array 
store 3 // this will store the current value onto the third variable  
var 3 // retrieves the same value as a copy so we dont lose value on top of the stack
push 3 // push 3 onto the top of the stasck 
var 3 // retrieves var 3 and places on the top of the stack 
push 0 // pushes zero onto the top of the stack
get // retrieves vbase[0] 
setframe 3  // sets fp to stack.len - i (3) - 1 
swap // swaps top two values on the stack 
call // this will call L6 
store 2 // now we store what was in var 1 from L6 (line 30), stores in var 2 of current fp
pop // removes top value from stack 
ret // this takes us back to line 4 where we halt 
_L6:
var 1 // retrieves from whatever var 1 is and returns that value 
ret // ret will take us back to line 26 