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