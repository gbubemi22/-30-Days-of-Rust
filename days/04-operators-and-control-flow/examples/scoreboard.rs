fn main() {
    let scores = [72, 45, 88, 60];
    let passing_score = 60;
    let mut passed = 0;

    for index in 0..scores.len() {
        let score = scores[index];
        let result = if score >= passing_score {
            "pass"
        } else {
            "retry"
        };
        println!("Score {score}: {result}");

        if score >= passing_score {
            passed += 1;
        }
    }

    let mut countdown = 3;
    while countdown > 0 {
        print!("{countdown} ");
        countdown -= 1;
    }
    println!("Go!");
    println!("Passing scores: {passed}");
}
