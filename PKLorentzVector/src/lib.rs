use PKVar;
use num_traits::Pow;
use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct lorentz_vec {
    X : [PKVar::var; 4],
    Magn_ : PKVar::var,
}

impl lorentz_vec {
    pub fn getMagnitude(&self) -> PKVar::var {
        self.Magn_
    }
}

pub fn PKLorentzVector(x0 : PKVar::var, x1 : PKVar::var, x2 : PKVar::var, x3 : PKVar::var) -> lorentz_vec {
    lorentz_vec {
        X : [x0, x1, x2, x3],
        Magn_ : (Pow::pow(x0, 2)-Pow::pow(x1, 2)-Pow::pow(x1, 2)-Pow::pow(x3, 2)).Sqrt()
    }
}

pub fn fromFloats(x0 : f64, x1 : f64, x2 : f64, x3 : f64) -> lorentz_vec
{
    let x_0 = PKVar::PKVar(x0, 0.);
    let x_1 = PKVar::PKVar(x1, 0.);
    let x_2 = PKVar::PKVar(x2, 0.);
    let x_3 = PKVar::PKVar(x3, 0.);

    PKLorentzVector(x_0, x_1, x_2, x_3)
}

pub fn fromFloatsAndErrors(x0 : f64, x1 : f64, x2 : f64, x3 : f64, 
                       x0_err : f64, x1_err : f64, x2_err : f64, x3_err : f64) -> lorentz_vec
{
    let x_0 = PKVar::PKVar(x0, x0_err);
    let x_1 = PKVar::PKVar(x1, x1_err);
    let x_2 = PKVar::PKVar(x2, x2_err);
    let x_3 = PKVar::PKVar(x3, x3_err);

    PKLorentzVector(x_0, x_1, x_2, x_3)
}

impl ops::Add<lorentz_vec> for lorentz_vec {
    type Output = lorentz_vec;

    fn add(self, _rhs : lorentz_vec) -> lorentz_vec {
        PKLorentzVector(
            self.X[0]+_rhs.X[0],
            self.X[1]+_rhs.X[1],
            self.X[2]+_rhs.X[2],
            self.X[3]+_rhs.X[3]
        )
    }
}

impl ops::Sub<lorentz_vec> for lorentz_vec {
    type Output = lorentz_vec;

    fn sub(self, _rhs : lorentz_vec) -> lorentz_vec {
        PKLorentzVector(
            self.X[0]-_rhs.X[0],
            self.X[1]-_rhs.X[1],
            self.X[2]-_rhs.X[2],
            self.X[3]-_rhs.X[3]
        )
    }
}

impl fmt::Display for lorentz_vec {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PKLorentzVar({:.4}, {:.4}, {:.4}, {:.4})", self.X[0], self.X[1],self.X[2], self.X[3])
    }
}

