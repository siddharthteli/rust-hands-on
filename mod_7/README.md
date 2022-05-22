# Heap & Stack
- Heap
  - If size of data type is unknown/ not fixed memory is allocated on heap & ptr is stored on stack.
  - Coz it requires mutiple call to stack & then to heap. It's slower to fetch value from heap. 
  - String's, arrays & structs are all stored on heap.
  - If we copy a type which is stored on heap, Rust just copies the ptr value & invalidates original type coz its value is moved. 

- Stack
  - All primitive types like float,int etc are by default stored on stack. 
  - Faster fething of values.


 - Copy & Drop
  - Rust won't let us use copy trait if a type has impl drop trait. 



  




