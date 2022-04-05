# Variables, Constants, Data Types, Functions, and Control Flow

## Variables

To declare a variable, we use a preserved keyword, **let**.

```rust
	let x = 10;
```

There are two types of variable: **immutable** and **mutable**. All variables we declared in rust are **immutable in default**. To make it mutable, we have to add another keyword **mut**. Although we can assign value to mutable variable freely, assigning a value with different type is illegal in rust.

```rust
	let mut x = 10;
	x = 100; 
	x = "test" // This will cause compile error.
```

But sometimes we want to reuse the same variable name and assign value without limitation of typing. To achieve this, we can use a mechanism call **shadowing**. Redefining the variable with the **let** keyword does this task.

```rust
	let x = 10;
	let x = x * 10;
	let x = "This is 10 in string";
```

## Constants

Constants are immutable values and it can never be changed. Constant is declared with a *const* keyword. Behind the names, we have to annotate the data type of the constants. Since the compiler can evaluate a limited set of operations at compile time, value that can only calculate in run time is not acceptable by constant. Constants are able to use in global scope. In addition, the naming convention of constant is uppercase with underscores between words. 

```rust
	const CONST_VALUE: i8 = 10;
	const SECONDS_IN_HOUR: u16 = 60*60;
```



