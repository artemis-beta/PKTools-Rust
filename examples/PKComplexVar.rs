use PKComplexVar;

fn main() {
    let x_r : f64 = 10.;
    let x_i : f64 = 2.;
    let y_r : f64 = 5.;
    let y_i : f64 = 1.;

    let X = PKComplexVar::PKComplexVar( x_r, x_i );
    let Y = PKComplexVar::PKComplexVar( y_r, y_i );

    println!("X = {}", X.returnString(0));
    println!("Y = {}", Y.returnString(0));

    let mut z = X+Y;

    println!( "\nAddition: ");
    println!("{}", z.returnString(0));

    z = X-Y;

    println!( "\nSubtraction: ");
    println!("{}", z.returnString(0));

    z = X*Y;

    println!( "\nMultiplication: ");
    println!("{}", z.returnString(0));

    z = X/Y;

    println!( "\nDivision: ");
    println!("{}", z.returnString(0));

    z = X*20;

    println!( "\nInt Multiplication (X*20): ");
    println!("{}", z.returnString(0));

    z = X/20;

    println!( "\nInt Division (X/20): ");
    println!("{}", z.returnString(0));
}