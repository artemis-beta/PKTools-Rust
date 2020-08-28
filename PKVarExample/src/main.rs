use PKVar;

fn main() {
    let x_v : f64 = 100.;
    let x_e : f64 = 10.;
    let y_v : f64 = 56.;
    let y_e : f64 = 0.34;

    let x = PKVar::PKVar( x_v, x_e );

    let y = PKVar::PKVar( y_v, y_e );

    println!( "Vector 1:" );
    println!( "{}", x );
    println!( "Vector 2:" );
    println!( "{}", y );

    let mut z = x+y;

    println!( "\nAddition: ");
    println!("{}", z);

    z = x-y;

    println!( "\nSubtraction: ");
    println!("{}", z);

    z = x*y;

    println!( "\nMultiplication: ");
    println!("{}", z);

    z = x/y;

    println!( "\nDivision: ");
    println!("{}", z);
}
