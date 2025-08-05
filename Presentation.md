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
```rust
    let s1: String = String::from("hello"); // From<&str> for String
    let s2: String = "world".into();        // Into<String> for &str

    println!("{s1} {s2}");
```   

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
## Mutable variables
```rust
  let mut x = 5;
  x = 10;
  println!("{}", x);
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

## While
```rust
fn factorial(n: u32) -> u32 {
  let mut result = 1;
  let i = 1;
  while i <= n {
    result *= i;
  }

  result
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
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

---
## Вытаскиваем элемент строки
```rust
  let strings = vec!["Hello, ".to_string(), "World!".to_string()];
  let hello = &strings[0];

  // let hello = strings[0];

  for x in &strings {
    println!("{x}");
  }
```
```rust
  let mut s = String::from("Hello");
  let c: &mut String = &mut s;

  let d = c; // ✅ компилируется, c больше использовать нельзя
  d.push_str(" world");
  println!("{d}");
```

## Struct

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 10, y: 20 };
println!("({}, {})", p.x, p.y);
```


```rust
struct Color(u8, u8, u8);

let red = Color(255, 0, 0);
println!("R = {}", red.0);
```

```rust
impl Point {
    fn new(x: i32, y: i32) -> Self { // конструктор
        Self { x, y }
    }

    fn distance(&self) -> f64 {      // метод
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    fn move_x(&mut self, dx: i32) {  // метод с &mut self
        self.x += dx;
    }
}


let mut p = Point::new(3, 4);
println!("{}", p.distance()); // 5
p.move_x(2);

```

```rust
enum Shape {
    Circle(f64),             // кортеж
    Rectangle { w: f64, h: f64 }, // структура
}
```

```rust
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle { w, h } => w * h,
        }
    }
}

let s1 = Shape::Circle(2.0);
let s2 = Shape::Rectangle { w: 3.0, h: 4.0 };
println!("area1 = {}", s1.area());
```

## Option and Result
```rust
fn divide_optional(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None           // нет результата
    } else {
        Some(a / b)    // результат есть
    }
}

if let Some(result) = divide_optional(10, 2) {
    println!("Результат: {result}");
} else {
    println!("Деление на ноль!");
}

```

```rust
fn divide_result(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Деление на ноль")
    } else {
        Ok(a / b)
    }
}


match divide_result(10, 0) {
    Ok(result) => println!("Результат: {result}"),
    Err(msg) => println!("Ошибка: {msg}"),
}

```
```rust
let x = divide_optional(10, 0).unwrap_or(999); 
println!("{x}"); // 999
```

Mutex, Arc -- на самом проекте.
Лямбды -- тоже на нём

---

# Generic
```rust
use num_traits::Float;
  struct Point<T> {
    x: T,
    y: T,
  }
  
  impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
      return Point { x, y };
    }
  }
  
  impl<T: Float> Point<T> {
    fn distance_to_origin(&self) -> T {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
  }
```

## Traits
```rust
trait Measurable {
    fn measure(&self) -> f64; // возвращаем "размер" как число
}
```
```rust
struct Segment {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl Measurable for Segment {
    fn measure(&self) -> f64 {
        let dx = self.x2 - self.x1;
        let dy = self.y2 - self.y1;
        (dx*dx + dy*dy).sqrt() // длина отрезка
    }
}

struct Circle {
    r: f64,
}

impl Measurable for Circle {
    fn measure(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r // площадь круга
    }
}
```


##
```rust
fn print_measure<T: Measurable>(item: &T) {
    println!("Measure = {}", item.measure());
}

fn main() {
    let seg = Segment { x1: 0.0, y1: 0.0, x2: 3.0, y2: 4.0 };
    let circle = Circle { r: 2.0 };

    print_measure(&seg);    // 5.0
    print_measure(&circle); // 12.566...
}
```


# Задачка

```rust
trait Movable {
    fn r#move(&mut self, dx: f32, dy: f32); 
    fn distance_to(&self, other: &Self) -> f32;
}

struct Particle {
    x: f32,
    y: f32,
    mass: f32,
    charge: f32,
}

impl Physical for Particle {
    fn mass(&self) -> f32 {
        self.mass
    }
    fn charge(&self) -> f32 {
        self.charge
    }
}

impl Movable for Particle {
    fn move_by(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

fn electrostatic_force<A: Physical + Movable, B: Physical + Movable>(a: &A, b: &B) -> f32 {
    let k = 8.9875e9; // Константа Кулона
    let q1 = a.charge();
    let q2 = b.charge();
    let r = a.distance_to(b).max(1e-6); // чтобы не делить на 0

    k * q1 * q2 / (r * r)
}


```
