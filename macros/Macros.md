# Macros
-------------------------------------------------
<div> <h4>
<ul>
<li> Macros provide a way of writing code that writes other code. </li>
<li> These are also sometime known as meta programming in programming community. </li>
<li> Macros in Rust are of two types: </li>
    <ol>
        <li> Declarative Macros </li>
        <li> Procedural Macros </li>
    </ol>
<li> Procedural Macros are further divided into,</li>
    <ol>
        <li> Custom Derive Macros </li>
        <li> Attribute-like Macros </li>
        <li> Function-like Macros </li>
    </ol>
</ul>
</h4> </div>

-------------------------------------------------
## Declarative Macros
-------------------------------------------------
- Declarative Macros are most common and widely used macros in Rust.
- Generally, in the community, when we talk about macros, we are referring to **declarative macros**.
- Declarative Macros are also known as **macro_rules!** macros, or **macros by example** or just plain **macros**.

=================================================
### Basic Syntax
=================================================
- The syntax of declarative macros is similar to that of match expressions.
- The syntax starts with reserved keyword of `macro_rules!` followed by the name of the macro and then body of the macros.
- Inside the body of the macro, we have the match rules. - Each macro rule is separated by a semicolon. Semicolon is optional in case of the ending or last rule.
- Each macro needs to have at least one rule and may contain many rules.
- The rules have similar syntax to that of match expressions.
- The left side is a matching pattern and the right side indicates the code substitution that needs be made when a pattern is matched.
- Let's define a simple macro to understand some basic syntax.

```rust
macro_rules! our_macro {
    () => {
        1 + 1
    };
}

pub fn main() {
    our_macro!();
}
```
- In the above example, we have defined a macro named `our_macro` which doesn't take any arguments and returns `1 + 1`.
- Let's understand what happens when we call this macro.
- `our_macro` has no associated pattern, so it will substitute the code `1 + 1` whenever it is called.
- While a function returns something, (when function returns nothing explicitly, it returns `()`), a macro substitutes the code in place of the macro call.
- So, when we call `our_macro!()`, it will substitute `1 + 1` in place of the macro call.
- We can check this by running the following code
```rust
macro_rules! our_macro {
    () => {
        1 + 1;
    };
}

pub fn main() {
    println!("{}", our_macro!());
}
```
- Here we have added `;` after `1 + 1` and we are printing the result of the macro call. When we run this code, it will print `2`. If it was a function call, it would not compile with `;` at the end of the function call.
- Now if we change the code to following
```rust
macro_rules! our_macro {
    () => {
        1 + 1;
    };
    (something nonesens ladfja0oasdjf 0) => {
        println!("You found the nonsense!");
    }
}

pub fn main() {
    println!("{}", our_macro!());
    our_macro!(something nonesens ladfja0oasdjf 0);
}
```
- We get the following output,
```shell
2
You found the nonsense!
```
- **In summary, left side contains matching expression and right side contains the rust code that needs to be substituted when the pattern is matched.**
- **The macro call is replaced by the code on the right side of the matched pattern.**
- **The left side of the pattern can be anything that is syntactically correct, i.e. can be parsed by the Rust compiler.**
- **The right side of the pattern can be any valid Rust code, with valid syntax.**
#### Captures
- Until now, pattern matching is nonsensical. But more useful patterns can be constructed by making use of something called `captures`.
- Captures are written as `$` followed by a name and then a colon, followed by the kind. The kind of the capture can be an expression (`expr`), a type (`ty`) or and identifier (`ident`).
- One of the basic type of capture is `expr`. It captures an expression.
- Let's see an example,
```rust
macro_rules! our_macro {
    () => {
        1 + 1
    };
    (something nonesens ladfja0oasdjf 0) => {
        println!("You found the nonsense!");
    };
    ($e1: expr, $e2: expr) => {
        $e1 + $e2
    }
}

pub fn main() {
    println!("{}", our_macro!());
    our_macro!(something nonesens ladfja0oasdjf 0);
    println!("{}", our_macro!(1, 2));
}
```
- In the third rule, we have two captures `$e1` and `$e2` which captures an expression. We are adding these two expressions in the right side of the rule.
- When we call `our_macro!(1, 2)`, it will substitute `1 + 2` in place of the macro call.
- **Macros match the exact syntax, so if we call `our_macro!(1, 2,)`, it will not match the pattern and will throw an error. Or any other syntax error will also throw an error.**
- Invocation should match at least one rule, otherwise it will throw an error.
```rust
macro_rules! our_macro {
    () => {
        1 + 1
    };
    (something nonesens ladfja0oasdjf 0) => {
        println!("You found the nonsense!");
    };
    ($e1: expr, $e2: expr) => {
        $e1 + $e2
    };
    ($e1: expr, $e2: expr, $e3: expr) => {
        $e1 * ($e2 + $e3)
    }
}

pub fn main() {
    println!("{}", our_macro!());
    our_macro!(something nonesens ladfja0oasdjf 0);
    println!("{}", our_macro!(1, 2));
    println!("{}", our_macro!(1, 2, 3));
}
```
- Suppose we call, `our_macro!("Shubhendu", 2, "Raj")`, it will match with the last rule, but it will throw an error because in this case expansion part will not result in valid rust code.
- Error is not thrown in `main()` but in the expansion part of the macro.
- **In summary, is that capture allows us to write some expanded code based on the items or to be specific, the kind of captures in the pattern syntax.**

=========================================================

- #### Let's look at some additional points,
    - Semi-colon is optional in the last rule.
    - We can any type of brackets when invoking the macros.
    - We can also choose any type of brackets in the rules of the macros, for both the left and right side. But the convention is to use `()` for the left side and `{}` for the right side.

=========================================================
- There is a very nice utility called `Cargo Expand` which provides, very useful insights into the macro expansion.
- This basically expands the expansion or substitution part of the macros and displays the expanded code.
- To use this utility, we need to install it first.
```shell
cargo install cargo-expand
```
- Once installed, we can use it as follows,
```shell
cargo expand
```
- This will display the expanded code of the macros in the terminal.
- **This is a very useful tool to understand the macro expansion.**

---------------------------------------------------------
### Capturing Types
---------------------------------------------------------
#### Using `ty` Capture
=========================================================
- We will start with special form of captures that is used for types.
- We will cover this with an example of taking input from terminal.
- Usually in Rust, reading input is slightly complex from other languages.
```rust
macro_rules! input {
    () => {
        let mut n = String::new();
        std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input.");

        let n = n.trim().parse().expect("Invalid input");
    }
}
```
- Last line of code in above macro is used for converting the input to the desired type. So we need to pass the type of the input to the macro.
- We can use special capture to do this, called `ty`. 
```rust
macro_rules! input {
    ($t: ty) => {
        let mut n = String::new();
        std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input.");

        let n: $t = n.trim().parse().expect("Invalid input");
        n
    }
}
```
- We can use the `$t: ty` capture to capture the type of the input and then use it in the code.
- We can use this `$t` in the code as we have used in the last line of the macro, to convert the input to the desired type.
- And in the last line of the macro, we are returning the input.
- We can use this macro as follows,
```rust
pub fn main() {
    println!("Please enter a floating point number:");
    let some_input = input!(f64);
}
```
- But the compiler is not happy with this code. It is throwing an error. The error is because when expanding the code, brackets `{}` are not part of the macro expansion. That means we have these lines as substitution in the macro expansion, and we are assigning lines of code to a variable.
- We can enclose the block of code in `{}` and then return the variable.
- Without introducing these extra brackets, we will have all these lines being substituted for the invocation, which doesn't correspond to any valid Rust code.
```rust
macro_rules! input {
    ($t: ty) => {{
        let mut n = String::new();
        std::io::stdin()
            .read_line(&mut n)
            .expect("Failed to read input.");

        let n: $t = n.trim().parse().expect("Invalid input");
        n
    }};
}

pub fn main() {
    println!("Please enter a floating point number:");
    let some_input = input!(f64);
    println!("You entered: {}", some_input);
}
```
- Now the code will compile and run successfully.
- **In summary, we can use `ty` capture to capture the type of the input and then use it in the code.**

=========================================================
- Let's look at another example of using `ty` capture.
- Let's define a macro for adding numbers in different formats or types with the name of `add_as`, which takes two expressions and one type.
```rust
macro_rules! add_as {
    ($a: expr, $b: expr, $type: ty) => {
        $a as $type + $b as $type
    };
}

fn main() {
    let a: i8 = 10;
    let b: f32 = 20.0;
    let c = add_as!(a, b, f64);
}
```
- This code will compile and run successfully.
- In absence of macros we would have to handle each type conversion manually, which will require more code and will be more error prone.
- **Idea behind `macros` are to make the code more readable by extracting out some of the unwanted details, and boilerplate code.**

---------------------------------------------------------
### Using `ident` Capture
=========================================================
- Identifiers are something in our program that has some name associated with it, such as variable names, function names, with the help of which we are able to identify something inside the code.
- We can capture identifiers using `ident` capture.
- Let's define a simple macro `some_macro`.
```rust
macro_rules! some_macro {
    () => { let mut x = 10;}
}

fn main() {
    some_macro!();
}
```
- Above code will compile with no errors.
- But if we try to mutate the variable `x` in the main function, it will throw an error.
```rust
fn main() {
    some_macro!();
    x = x + 1;
}
```
- `Cannot find value x in this scope` error will be thrown.
- But this should work fine? Right? Because we have defined `x` in the macro, and macros are expanded at compile time.
- This is because the identifier, which in this case is the variable `x` that is inside the macro and in the main function are just completely different. They are not allowed to cross the boundaries, i.e. the scope of the macro. That's why they are also called hygiene macros.
- This is where `ident` capture comes into play.
- They allow you to cross the boundaries which will otherwise may not be possible.
- In general, variables and identifiers in outside the macro world cannot be transferred or shifted to the inside of the macro world. But with the help of `ident` capture, we can do this.
- Let's see an example of this.
```rust
macro_rules! some_macro {
    ($var: ident) => { $var = $var + 1;}
}

fn main() {
    let mut x = 10;
    some_macro!(x);
    println!("{}", x);
}
```
- In the above code, we are passing the variable `x` to the macro and then mutating it inside the macro.
- This code will compile and run successfully.
- **In summary, `ident` capture allows us to pass the identifiers from the outside world to the inside world of the macro.**

---------------------------------------------------------
### Ownership in Macros
=========================================================
- The macros doesn't take ownership of something, you have to keep and eye on the expansion only.
- **This means that macro has nothing additional to do with ownership of variables and they retain their ownership as they would have in the normal code.**

---------------------------------------------------------
### Final Example with all the concepts
=========================================================
- We will define a macro which will create a function for us.
```rust
macro_rules! create_function {
    ($func_name: ident, $input: ident, $type: ty) => {

        fn $func_name($input: $type) -> $type {
            println!("You called {:?)() with the input of {:?}", stringify!($func_name), stringify!($input));
        }
    }
}
```
- Let us now invoke the macro in our program since the definition of the functions are outside the main function.
```rust
create_function(f1, x, i32);
create_function(f2, y, f64);

fn main() {
    let x = 10;
    let y = 20.0;

    let a = f1(x);
    let b = f2(y);
}
```
---------------------------------------------------------
### Repeating Patterns in Macros
---------------------------------------------------------
- `Repeating Patterns` are essentially patterns that are repeating and we would like to make substitutions for each of the repeated patterns.
- Let's consider an example of a macro, that will concatenate multiple strings.
- There can be any number of input strings and we would like to concatenate them.
- If there are no input strings, we would like to return an empty string.
- Repeated sequences are are mentioned using `$(...),`.
- Brackets inside `$(...),`, i.e. in place of `...` are used to capture the repeated patterns. And `,` is used to denote the delimiter. It can be any delimiter, like `;`, `:` etc.
- After the delimiter `,`, we can use `*` to denote that the pattern can be repeated zero or more times. `+` can be used to denote that the pattern can be repeated one or more times. `?` can be used to denote that the pattern can be repeated zero or one time.
- 
```rust
macro_rules! string_concat {
    () => { String::new() };
    // ($some_str: expr) => {{
    //     let mut s = String::new();
    //     s.push_str($some_str);
    //     s
    // }};  // Last match can be used for above two matches
    ($($some_str: expr), +) => {{
        let mut s = String::new();
        $(
            s.push_str($some_str);
        )*
        s
    }};
}

pub fn main() {
    let null_str = string_concat!();
    println!("Null String: {}", null_str);

    println!();
    println!("Using one string:");
    let some_str = string_concat!("Hello, World!");
    println!("Some String: {}", some_str);

}
```
- In the above code, we have defined a macro `string_concat` which takes any number of strings and concatenates them.

=========================================================
#### Additional Information regarding delimiters and repetitions operators
=========================================================
- **Delimiters**
    - We can remove the `,` from repeating pattern and it will still work, because it will use default delimiter ` `, i.e. space.
    - Generally the convention is to use `,` as delimiter.
    - We can use any delimiter, like `;`, `:` etc.
    - We can also add the delimiter to the repeated sequence pattern, like `$(...;),`. In this case the delimiter will be `;`, and compiler will enforce that the pattern should be separated by `;`, and each expression should end with `;`, even last one.
    - Repetition pattern which we use to match should be consistent with pattern used in expansion part.
    