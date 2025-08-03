Introduction to rust

--- 

# Basic types and constructions

## Basic types:
1. `i32`, `i64`, ...
2. `u32`, `u64`, ...
3. `f32`, `f64`, ...
4. `bool`
5. `char`

---

## Types conversions:
1. Use `as` for primitives 
```rust
  let x: i32 = 2;
  let y: i64 = x as i64;
```

2. Use T::from, T::into for complex types

---
## Variable creation

```rust
let x : i32 = 1;
println!("{}", x);
```

```rust
println!("{}");
println!("{:?}");
println!("{:#?}");


```

---

## Tuples
1. `.0`, `.1`
2. Copy-type if members are copy-types
3. Use `{:?}` to print them

```rust
let p = (32, 'a')
let (x, c) = p
```

---

## If-else
```rust

  if (x > 0) {
    println!("Hey, x > 0");
  }

  let abs_x = if x > 0 { x } else { -x };
```
---
## Math function
```rust
2.powi(10);
.powf(10.2);
.sin();
.sqrt()
PI
.max, .min
```


---
## Custom functions

1. Void = ()
2. `return x;` <=> `x` without `;`

```rust
fn compute_distance_2d(p1: (f32, f32), p2: (f32, f32)) -> f32 {
  let (x1, y1) = p1;
  let (x2, y2) = p2;
  ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

```

---

## Match
```rust
fn choose_action(symbol: char, op1: f32, op2: f32) -> f32 {
  match symbol {
    '+' => op1 + op2,
    '-' => op1 - op2,
    '*' => op1 * op2,
    _ => panic!("Unrecognized operation!"),
  }
}
```

```rust
fn is_edge_chess_coordinate(x: char, y: i32) -> bool {
  match (x, y) {
    (0, _) => true,
    (8, _) => true,
    (_, 'a') => true,
    (_, 'h') => true,
    _ => false
  }
}
```

## Mutable and ownership
```rust
fn main() {
  let mut x = 2;
  println!("x = {}", x);

  x = 5;
  println!("x = {}", x); -- ok
}
```

## References and mutability

```rust
fn main() {
  let mut x = 5;
  let mut y = x;
  y += 10;

  println!("x = {}, y = {}", x, y);
  // x = 5, y = 15
}
```
```rust
fn main() {
  let mut x = 5;
  let y = &mut x;
  *y += 10;

  println!("x = {}", x);
  // println!("x = {}, y = {}", x, y) -- does not work!
}
```

```rust
fn main() {
  let reference: &i32;

  {
    let value = 10;
    reference = &value;
  }

  println!("{:?}", reference);
}
```


Про владение поговорить на второй паре?
Строки, структуры, traits, ссылки, владение

---

# Structs and collections

## Vec<T>

```rust
  let mut numbers = vec![0, 1, 2, 3];
  for x in numbers {
    println!("{}", x)
  }
```

```rust
  let mut numbers = vec![0, 1, 2, 3];
  
  // Use &numbers
  for x in numbers {
    println!("{}", x)
  }

  numbers.push(4); // not ok, use &numbers

```
---
## References and dereferencing

How to modify an element of a vector?

```rust
fn main() {
  let mut numbers = vec![0, 1, 2, 3];

  for x : &mut i32 in numbers.iter_mut() {
    *x += 1; // dereferencing
  }
  println!("{:?}", numbers);
}
```

## Borrowing
1. At any moment, exactly one variable is responsible for allocated memory.
2. There could be at most one mutable reference at the time, in this case even non-mutable references are not permitted.
3. The value is destroyed when there are no ownership

```rust
fn main() {
  let numbers = vec![1, 2, 3];
  let same_numbers = numbers;

  println!("{:?}", numbers) // does not work, use .clone()
}

```


## Passing to functions and mutable iterations

1. Use &mut Vec<i32> as a type for parameter
2. Use .iter_mut()
3. Use * to access the value by reference

```rust

fn add_x(numbers: &mut Vec<i32>, x: i32) -> &Vec<i32> {
  for y in numbers.iter_mut() {
    *y += x;

    // Не сработает такое:
    // let z = *y;
    // z += x;
  }
  numbers
}


fn main() {
  let mut numbers = vec![0, 1, 2, 3];

  add_x(&mut numbers, 5);
  println!("{:?}", numbers);
}

```

# Struct, enum, Optional, Result, 

Mutex, Arc -- на самом проекте.
Лямбды -- тоже на нём



