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

Constants are immutable values and it can never be changed. Constant is declared with a *const* keyword. Behind the names, we have to annotate the data type of the constants. Since the compiler can evaluate a limited set of operations at compile time, value that can only calculate in run time is not acceptable by constant. Constants are able to use in **global scope**. In addition, the naming convention of constant is uppercase with underscores between words. 

```rust
	const CONST_VALUE: i8 = 10;
	const SECONDS_IN_HOUR: u16 = 60*60;
```

## Data Types

Rust is a **statically typed** language which means all variables must have type. As what we mentioned in the previous paragraph, it is not necessary to annotate the data type of a varible declared with *let*. When we assign a value to a variable by *let*, the data type of a variable is automatically decided.

```rust
	let x = 100; 	// x variable is integer
	let y = "test"; // y variable is string
	let z = 100.0;	// z variable is float
```

### Scalar Types

Scalar type represents a single value. There are:
1. integer
2. floating-point
3. boolean
4. character

#### Integer

There are two subclass beneath integer: signed and unsigned. Like C, the size of integer depends on the bit length of data: 8, 16, 32, 64 and 128 bits.

|Length|Signed|Signed Range|Unsigned|Unsigned Range|
|---|---|---|---|---|
|8 bits|i8|-2<sup>7</sup>~2<sup>7</sup>-1|u8|0~2<sup>8</sup>|
|16 bits|i16|-2<sup>15</sup>~2<sup>15</sup>-1|u16|0~2<sup>16</sup>|
|32 bits|i32|-2<sup>31</sup>~2<sup>31</sup>-1|u32|0~2<sup>32</sup>|
|64 bits|i64|-2<sup>63</sup>~2<sup>63</sup>-1|u64|0~2<sup>64</sup>|
|128 bits|i128|-2<sup>127</sup>~2<sup>127</sup>-1|u128|0~2<sup>128</sup>|

In addition, rust has two special expansions for integer type: **usize**, **isize**. This will create an integer type depending on your running architecture. For example, isize is 32-bit signed integer for a x86-64 architecture. 

There are 5 literal expression for an integer in rust, there are: **decimal**, **hex**, **octal**, **binary** and **byte**. The special case is that byte only supports **u8** values, and they are expressed in b'\<ascii\>'.

|Literal|Example for 1000|
|---|---|
|Decimal|1_000|
|Hex|0x3E8|
|Octal|0o1750|
|Binary|0b11_1110_1000|


#### Floating-point

Floating-pointer numbers are following IEEE-754. There are two types: **f32**, a single-precision float, and **f64**, a double-precision float. All default float numbers are **f64** in rust. 

#### Boolean

**true** and **false**.

#### Character

char in c is only 1 byte so it can only accept ascii code. However, rust's char can accept scalar values of unicode ranging from U+0000 to U+D7FF and U+E000 to U+10FFFF. 

### Compound Types


