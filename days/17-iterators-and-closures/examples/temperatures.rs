fn main() {
    let readings = vec![12, 18, 21, 9, 25];
    let comfortable: Vec<i32> = readings
        .iter()
        .copied()
        .filter(|temperature| *temperature >= 18)
        .map(|temperature| temperature + 1)
        .collect();

    println!("Adjusted comfortable readings: {comfortable:?}");

    let labels: Vec<String> = readings
        .iter()
        .map(|temperature| format!("{temperature}°C"))
        .collect();
    println!("All readings: {labels:?}");
}
