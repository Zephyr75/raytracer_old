# Rust basics

## Variables

    let x: i32 = 42;

**Mutable** variable: let mut x

Possibly **unused** variable: _x

---
## Types

**Tuple**: pair = (a, b)
-  Access: pair.0, pair.1
-  Destructure: (x, y) = pair

**Array**: let v = Vec::new(); <br>
Ranges:

    (0..) → 0 or greater
    (..20) → strictly less than 20
    (..=20) → less than or equal to 20
    (3..6) → 3, 4, 5

**Generic type**:

    struct Point<T> {
        x: T,
        y: T,
    }

Type parameter **constraint**: <T: Display> → T must implement Display <br>
Also written: <T: Display> () where T: Display + Debug,

---
## Structs
    struct Vec2 {
        x: f64,
        y: f64,
    }

Initialize rest of the fields from other struct:

    let v3 = Vec2 {
        x: 14.0,
        ..v2
    };

Declare function for struct:

    impl Number {
        fn is_strictly_positive(self) -> bool {
            self.value > 0
        }
    }

---
## Traits
Traits are functions common to multiple types.

    trait Signed {
        fn is_strictly_negative(self) -> bool;
    }
    impl Signed for Number {
        fn is_strictly_negative(self) -> bool {
            self.value < 0
        }
    }


---
## Functions
    fn main() {
        let x = 42;
        println!("{}", x);
    }

Declare **return type**: fn main() -> i32 {42}

---
## Macros
Differentiated from functions by the **bang** operator:

    println!("{}", x);

---
## Option

    let o1: Option<i32> = Some(128);
    o1.unwrap(); → returns 128

    let o2: Option<i32> = None;
    o2.unwrap(); → panic

Result(i32) → Ok("42") or Err("error")<br>
? → Returns error if not Ok()

---
## Closure

    |i: i32| -> i32 { i + 1 }

---
## Enums
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

---
## Lifetime
**'static** : Exists for the entire program lifetime

---
## Print
    println!("{}", x);

Prints error and displays line where the error happened:

    panic!("This panics");

---
## Import
    use std::cmp::min;
    use std::cmp::*;
    use std::cmp::{min, max};

---
## Attributes
Mark function as runnable during tests:

    #[test]
    fn check() {
        assert_eq!(2, 1 + 1);
    }


---
---
Check: lifetime