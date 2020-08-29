use PKLorentzVector;

fn main() {
    let x_0 : f64 = 100.;
    let x_1 : f64 = 20.;
    let x_2 : f64 = 5.;
    let x_3 : f64 = 2.;

    let P_X = PKLorentzVector::fromFloats( x_0, x_1, x_2, x_3 );
    let P_Y = PKLorentzVector::fromFloats( x_0, x_2, x_3, x_2 );

    println!( "Lorentz Vector 1:" );
    println!( "{}", P_X );
    println!( "Lorentz Vector 2:" );
    println!( "{}", P_Y );

    let mut z = P_X+P_Y;

    println!( "\nAddition: ");
    println!("{}", z);

    z = P_X-P_Y;

    println!( "\nSubtraction: ");
    println!("{}", z);

}