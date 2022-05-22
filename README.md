# Rust -hands -on
- Mod_1
  - Init enum from cli arguments
  - Impl display trait for enum

- Mod_2
  - Generate random num b/w 1..100
  - User guess the num
  - Tell user if guessed num is larger, smaller then   random num
  - If its equal then print congratulation & exist.
  - Loop while guess is not correct.

- Mod_3
  - Workspace have single parent toml file defining all package in the workspace.
  - Each workspace have seperate toml file .
  - If multiple package have same dependencies it would be installed only once.
  - All package will have single target file.

- Mod_4
  - Link https://www.codechef.com/problems/HMAPPY2
  - First line takes num of test cases
  - Second line takes four space seperated input
  - Third line is output if we win or lose

- Mod_5
  - Impl array & tupple
  - If in a let Statement
  - Labeled loop

- Mod_6
  - Basic difference between string & literal
  - Strings are stored in heap.
  - While literal are directly embeded on exe file. 

- Mod_7
  - Heap
  - If size of data type is unknown/ not fixed memory is allocated on heap & ptr is stored on stack.
  - Coz it requires mutiple call to stack & then to heap. It's slower to fetch value from heap. 
  - String's, arrays & structs are all stored on heap.
  - If we copy a type which is stored on heap, Rust just copies the ptr value & invalidates original type coz its value is moved. 

  - Stack
  - All primitive types like float,int etc are by default stored on stack. 
  - Faster fething of values.