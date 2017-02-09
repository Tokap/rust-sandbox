// For Loops:
for x in 0..10 {
    println!("{}", x); // x: i32
}

// Making enumeration happen in loops:
for (index, value) in (5..10).enumerate() {
    println!("index = {} and value = {}", index, value);
}

// OUTPUTS:
// index = 0 and value = 5
// index = 1 and value = 6
// index = 2 and value = 7
// index = 3 and value = 8
// index = 4 and value = 9

// A Slightly Different Example:
let lines = "hello\nworld".lines();

for (linenumber, line) in lines.enumerate() {
    println!("{}: {}", linenumber, line);
}

// OUTPUTS:
// 0: hello
// 1: world

// Breaking a loop:
let mut x = 5;

loop {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 { break; }
}

// Continuing: continue is similar, but instead of ending the loop, it goes to the next iteration.
// This will only print the odd numbers:
for x in 0..10 {
    if x % 2 == 0 { continue; }

    println!("{}", x);
}

// Being Specific About Break Points:
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
        if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
        println!("x: {}, y: {}", x, y);
    }
}

/********************************************/
/*********       Vectors      **************/
/******************************************/

// Cannot index using an i32 -> needs to be a usize
