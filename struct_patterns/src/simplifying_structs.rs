//--------------------------------------------------------------------------------------------------
//          Simplifying Structs
//--------------------------------------------------------------------------------------------------
// struct A {
//     f1: u32,
//     f2: u32,
//     f3: u32,
// }

// fn fn1(a: &mut A) -> &u32 {
//     &a.f2
// }

// fn fn2(a: &mut A) -> u32 {
//     a.f1 + a.f3
// }

// fn fn3(a: &mut A) {
//     let _x = fn1(a);
//     let _y = fn2(a);
//     // Following will cause an error, because x is borrowed as immutable
//     // println!("{x}");
// }

// Using decomposition to borrow independently

struct A {
    b: B,
    c: C,
}

struct B {
    f2: u32,
}

struct C {
    f1: u32,
    f2: u32,
}

fn fn1(a: &mut B) -> &u32 {
    &a.f2
}

fn fn2(a: &mut C) -> u32 {
    a.f1 + a.f2
}

fn fn3(a: &mut A) {
    let x = fn1(&mut a.b);
    let _y = fn2(&mut a.c);
    println!("{x}");
}

pub fn main() {
    let mut a = A {
        b: B { f2: 10 },
        c: C { f1: 20, f2: 30 },
    };
    fn3(&mut a);
    

}