fn main() {
    let distance_km: f64 = 12.5;
    let stops: u32 = 3;
    let is_open = true;
    let symbol = 'R';

    let location: (&str, i32, i32) = ("Park", 4, 9);
    let (place, x, y) = location;
    let route = ["Park", "Museum", "Library"];

    println!("Distance: {distance_km} km across {stops} stops");
    println!("Open: {is_open}; marker: {symbol}");
    println!("{place} is at ({x}, {y})");
    println!("Route starts at {} and ends at {}", route[0], route[2]);
}
