// Function to calculate sine using the Taylor series expansion
fn sine_taylor_series(x: f64, terms: u32) -> f64 {
    let mut result = 0.0;
    let mut power = x;
    let mut factorial = 1;
    let mut sign = 1.0;

    for i in 0..terms {
        result += sign * power / factorial as f64;
        power *= x * x; // Move to the next power of x (x^3, x^5, etc.)
        factorial *= (2 * i + 2) * (2 * i + 3); // Calculate factorial for the next term
        sign *= -1.0; // Alternate the sign
    }

    result
}

fn main() {
    let angle = 1.0; // Angle in radians
    let terms = 10;  // Number of terms in the Taylor series

    let sine_approx = sine_taylor_series(angle, terms);
    println!("The approximate sine of {} is {}", angle, sine_approx);
}
