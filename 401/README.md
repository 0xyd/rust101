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


## References
1. [What is ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)