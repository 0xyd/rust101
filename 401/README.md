# Ownership

Ownership is a set of rules that Rust uses to manage memory. According to rust book, there are 3 rules of ownership:

1. Variable is the *owner* of value.
2. Value can only have one owner at a time.
3. When owner leaves the valid scope, the value will be dropped. 

## Variable Scope

Variable exists in the scope where it is created.

```rust
{
    let a = "a";
    println!("{:?}", a);
}
// println!("{:?}", a); // Bug! "a" not found in this scope.
```

However, the variable initiates at the outer scope is still valid in the inner scope.

```rust
{
let b = "b";
  	{
        println!("{:?}", b); // b variable exists here.
    }
}
```

## Memory Allocation 

Size of Variable with types like u32, i32, and etc is known in compiled time. When function is call, these variables are allocated to stack. Some variables like String or even more complicated one like struct are allocated to head since rust doesn't know their exact size in compiled time. 

Due to different memory allocations, the way to handle these variables are also different. In the below example, we have two cases. All cases reference a variable with another variable but their data types are non-identical (`i32` vs `String`). When rustc compiles, it complains that "value (s1) borrowed here after move" in the case 2 so s1 is no longer valid.

```rust
	// Case 1: 
    let x = 1;
    let y = x;

    println!("x: {:?}, y: {:?}", x, y);

    // Case 2
    let s1 = String::from("test");
    let s2 = s1;

    // rustc complains
    // println!("s1: {:?}, s2: {:?}", s1, s2); 
```

According to the previous example, rustc uses `deep copy` when we assign a variable to another in stack. While in heap, rustc doesn't simply `shallow copy` but `move`. In shallow copy, compiler copies only pointer, length and capacity but no data. Move does one more thing after shallow copy: it invalidates the variable previously copied from.

To let case 2 work, we have to clone the whole data to the new variable by function `clone`:

```rust
	// Case 3
    let s3 = String::from("test");
    let s4 = s3.clone();
    println!("s3: {:?}, &s3: {:p}", s3, &s3);
    println!("s4: {:?}, &s4: {:p}", s4, &s4);   
```  

`deep copy` is actually implemented by a function named `copy` in rust. The following types support the function natively:
* Integer types
* boolean
* Floating point types
* char
* tuples composed by the above types

We can implement a copy function in our customized types too. But this is not going to be covered here.

## How Functions affect Ownership

When a variable is pass to a function, its ownership changes. Here we create 2 variables: one is mutable and the other one is immutable. Both variables pass to a function to add 1. The rustc can compile the code correctly but has a warning: help: remove this `mut`. This warning is very counter-intuitve because the acceptable parameter `x` in the `add_1` function is mutable. However, we need to keep in mind: when a value is passed to a function, its ownership changed to the parameter of the received function. Thus, the parameter can decide the value is mutable or not. 
```rust
{
	let mut c = 100;
    add_1(c);

    let d = 1;
    add_1(d);
}
fn add_1(mut x: i32){
    x = x+1;
    println!("{:?}", x);
}
```

Besides, the types of variables determine the ownership as well. As we have seen from the previous samples, some types like integers do `copy`. Types like string do `move` instead. The mechanism is the same when we pass a variable to a function. The following example shows that string *b* cannot be reused after its ownership is taken by function *takes_ownership*. In contrast, variable *a* can be reused in *main*.

```rust
fn main() {
    let a = 10; 
    let b = String::from("TAIWAN NO 1.");

    makes_copy(a);
    takes_ownership(b);

    println!("a {:?} is still workable in main.", a);
    // println!("{:?}", b); rust complains here since ownership is taken
}

fn makes_copy(x: i32) {
    println!("{:?} is coppied.", x);
}

fn takes_ownership(s: String) {
    println!("{:?} 's ownership is taken", s);
}
```

Since the ownership of the string is taken, we can manipulate the string in the function too.
```rust
fn main() {
	let c = String::from("1 + 2 = ");
    manipulates_string(c);

}

fn manipulates_string(mut s: String) {
    s.push('3');
    println!("{:?}", s);
}
```

One way to return the ownership to a variable in the *main* is by **return**.

```rust
{
	let d0 = String::from("1 + 2 = ");
    let d1 = rmanipulates_string(d0);
    println!("{:?}", d1);
}
fn rmanipulates_string(mut s: String) -> String {
    s.push('3');
    return s;
    // s ; // This is another expression for return 
}
``` 

## Reference

Reference is a pointer that points to a specific data type. 

With reference, ownership of a variable is borrowed by a function instead of being taken. We pass the reference of string *s0* to a function that calculates its length in the below example. We find out *s0* is still workable after the function is called.

```rust
{
	let s0 = String::from("Taiwan no 1.");
    let s0_len = calc_str_len(&s0);
    println!("{:?}", s0);
    println!("Length of s0: {:?}", s0_len);
}
fn calc_str_len(s: &String) -> usize {
    s.len()
}
```

Sometimes, we want to modify a value that a reference refers to in a function. We will use **mutable reference** to achieve this goal. First we need to declare the variable as a mutable in caller;  

```rust
{
	let mut s1 = String::from("Hello ");
    println!("s1 before world added {:?}", s1);
    add_world(&mut s1); // &mut tells rustc the referred value is mutable.
    println!("s1 after  world added {:?}", s1);
}

fn add_world(s: &mut String) { // Parameter must be declared as a mutable reference explicitly
    s.push_str("World!");
}
```

Rust has restriction on borrowing a mutable reference for multiple times. When we compile the following code, rustc complains that we cannot borrow a mutable variable for more than one time. This prevents the program from race condition.

```rust
{
	let mut s1 = String::from("Hello "); 
	let r1 = &mut s1;
    let r2 = &mut s1;
    // println!("r1 {:?}, r2 {:?}", r1, r2); The compiler complains: "cannot borrow `s1` as mutable more than once at a time"
}
```

Similarily, once a mutable variable borrowed by an immutable reference. Any mutable borrowing afterward is not allowed.

```rust
{
	let mut s2 = String::from("Small ");
    add_world(&mut s2);
    println!("s2 :{:?}", s2);
    let r3 = &s2;
    let r4 = &s2;
    println!("r3: {:?} , r4: {:?}", r3, r4);
    // let r5 = &mut s2; // rustc starts complaining
    // add_world(&mut s2); // rustc complains again
}
```

The restrictions only applies when the references are in the same scope. So we can create a new scope to apply the above mechanisms:

```rust
{   // Multiple borrowing with additional scope
	let r1 = &mut s1;
    // println!("r1 {:?}, r2 {:?}", r1, r2);

    {
        let r3 = &mut s1;
        add_world(r3);
        println!("In scope: {:?}", r3);
    }
    println!("After r3 called in scope, s1 :{:?}", s1);
}
```
```rust
{
	// A mutable borrowing after an immutable one.
	let mut s2 = String::from("Small ");
    let r3 = &s2;

    // let r5 = &mut s2; // rustc complains 
    // add_world(&mut s2); // rustc complains

    {
        let r5 = &mut s2;
        add_world(r5);
        println!("r5: {:?}", r5);
    }
    println!("After r5 called in scope, s2 :{:?}", s2);
```


## References
1. [What is ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
2. [No mutable parameters](https://www.snoyman.com/blog/2020/05/no-mutable-parameters-in-rust/)