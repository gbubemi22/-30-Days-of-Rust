# Day 10: Enums and Pattern Matching — solutions

Try the [exercises](../days/10-enums-and-pattern-matching/README.md#exercises) first. These are sample answers; equivalent variant names and messages are fine.

## 1. Traffic lights

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn print_action(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Prepare to stop"),
        TrafficLight::Green => println!("Go"),
    }
}
```

Each variant is handled, so this match is exhaustive.

## 2. Shapes with data

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn describe(shape: Shape) {
    match shape {
        Shape::Circle { radius } => println!("Circle radius: {radius}"),
        Shape::Rectangle { width, height } => {
            println!("Rectangle: {width} by {height}");
        }
    }
}
```

## 3. Match an `Option`

```rust
let score = Some(88);
match score {
    Some(value) => println!("Score: {value}"),
    None => println!("No score"),
}
```

For a missing score, set `score` to `None` and annotate it as `Option<i32>` if Rust needs the type.

## 4. Use `if let`

```rust
let score = Some(88);
if let Some(value) = score {
    println!("Score: {value}");
}
```

This runs the block only for `Some`; it does nothing for `None`.

## 5. Add a traffic-light variant

For example, add `Flashing` and add a corresponding arm such as `TrafficLight::Flashing => println!("Proceed with caution")`. Without that arm (or a catch-all), the compiler reports that the match does not cover every variant.
