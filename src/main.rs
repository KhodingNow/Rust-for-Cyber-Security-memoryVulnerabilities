// DEMO : Secure memory handling with Vec<T> in Rust

/*
- memory corruption bugs
- Buffer overflows
- Null pointer dereferencing 
- Use-after-free vulnerabilities
- Race conditions.
 
= these are ROOT causes of many critical exploits

*/

fn main() {
    println!("Memory Safety with Vec<T>!");

    // 1. Safe Vector initialization

    let mut secure_vec = vec![1, 2, 3, 4];
    println!("Original Vec: {:?}", secure_vec);
    
    // 2. Safe access with indexing
    println!("Elements at index 0: {}", secure_vec[1]);
  
    // 3. Try to access out of bounds
    //println!("Not a valid access: {}", secure_vec[5]); // comment out this line
    // this panics at runtime, protecting a Developer from themselves.

    // 4. Safe iteration (no buffer overread)
    // println!("Iterating over Vec:");
    // for val in &secure_vec {
    //     println!("Value: {}", val);
    // }

    // 5. Ownership and memory safety

    let moved_vec = secure_vec;
    //println!("{:?}", secure_vec); // Compile time ERROR: use after move
    println!("Moved Vec is still valid: {:?}", moved_vec);

    // 6. Borrowing and Compile time Race Avoidance

    let v = vec![1, 2, 3];
    let r1 = &v;
    println!("Immutable ref: {:?}", r1);

    // let r2 = &mut v // Won't compile, bcs we have a mutable borrow after an immutable borrow

    // 7. Unsafe : DO NOT DO THIS!

//     let dangerous_vec = vec![5, 6, 7];

//     unsafe {
// //        let ptr = dangerous_vec.as_ptr().add(10); // Out of bounds pointer
//         //println!("Dangerous access: {}", *ptr); // Unsafe and undefined
//     }

//     println!("Demo complete: Memory safety without a garbage collector!");

/* 

THEN... we escalate priviledges with Closure and Rust specialist methods

- Higher order functions while still respecting ownership, lifetimes, borrowing
- Show how data flows through Closures safely - no memory leaks or trigger of race conditions
- Show how Rust's iterator avoids temporary allocations and side effects
    
*/ 

// fn main() {
    println!("Secure Rust Demo: Memory Safety + Closures + Iterators\n ");

// 1. A simple vector
    let mut data = vec![200, 400, 600];
    println!("Original Vec: {:?}", data);

// 2. Closure with .iter() for read only, this method does not mutate, move, own data.
    println!("\n .iter() with closure (read-only, can't move, change, only borrows the data)");
    data.iter().for_each(|val| println!("Secure read: {}", val));

// 3. Closure with .into_iter() - this method can move, mutate and OWNS the data
    let owned_vec = vec![1, 2, 3, 4, 5, 6];
    println!("\n .into_iter() with closure OWNERSHIP - can move, change data inside the heap");
    let squared: Vec<i32> = owned_vec
        .iter()
        .into_iter()
        .map(|x| x * x)
        .collect();

    println!("Squared result: {:?}", squared);
    // println!("{:?}", owned_vec); // Error: moved, prevents use-after-free
    
// 4. map() + collect() avoids intermediate allocations
    let doubled = vec![5, 6, 7, 8]
        .iter()
        .map(|x| x * 3)
        .collect::<Vec<_>>();

    println!("\n .iter() + .map() + .collect(): doubled = {:?}", doubled);

// 5. Show safety in borrowed context

    let secured = vec!["admin", "user", "root"];
    println!("\n Checking the length safely with a closure:");
    secured.iter().for_each(|role| {
        println!("Role: {}, length: {}", role, role.len());
    });

// 6. Induced error - invalid memory location acces

    let vet = vec![2, 4, 6, 8];
    // println!("{}", vec[1001]); // Panic: prevents overflowing the buffer 

// 7. Rust prevents race conditions with closure scope
    
    let sensitive = vec![40, 1332, 800];
    let sum: i32 = sensitive.iter().copied().sum();
    println!("\n Sum is calculated in a safe way: {}", sum);
    
    println!("\n Closure based operations completed with memory safety, no Undefined Behaviour");


//}

// a Rust Closure protects when elements in the Vec are less than the 'for loop range' iteration instruction

// fn main() {
        let new_vec = vec![8, 9, 0, 3, 4, 6, 7, 11, 3, 5, 2, 20, 13, 17]; // just a vec with numbers
        println!("Contains vec elements: {:?}", new_vec);


        let number_to_add = 5; // use this in the maths later
        let mut empty_vec = vec![];

        for index in 0..20 {
            empty_vec.push(
                new_vec
                    .get(index)
                    .and_then(|number| Some(number + 1))
                    .and_then(|number| Some(number + number_to_add))
            );
        }
        println!("{:?}", empty_vec);        
   // }   

}
