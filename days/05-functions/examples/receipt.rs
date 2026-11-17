fn line_total(price: f64, quantity: u32) -> f64 {
    price * quantity as f64
}

fn print_receipt(item: &str, price: f64, quantity: u32) {
    let total = line_total(price, quantity);
    println!("{item}: {quantity} × {price:.2} = {total:.2}");
}

fn main() {
    print_receipt("Notebook", 2.50, 3);
    print_receipt("Pen", 1.25, 2);
}
