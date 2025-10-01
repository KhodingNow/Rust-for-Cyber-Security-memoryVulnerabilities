Adding a Rust lint (Clippy) onto my Rust in CyberSecurity - memory management codeBase - what is Clippy
- A lint built on top of a compiler
  It analyzes code and looks for:
1. 1. 1. Correctness issues (things that might be bugs) - e.g If x = y {} -> 'did you mean == ?'
2. 2. 2. Performance traps - e.g , using "vec.clone()" where a reference would do
   3. 3. Readability improvements. - e.g, replacing '.and_then(|x| Some(...) with '.map(...)'
   4. 4. Idiomatic style nudges - e.g, println!("Value: {:?}, x) -> println!("Value: {x:?}")



WHAT CLIPPY is NOT!
- it does not rewrite code
- it does not restructure entire modules, rename variables, or re-order functions
