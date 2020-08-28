use std::ops;
use std::fmt;
use num_traits::Pow;

#[derive(Copy, Clone)]
pub struct var {
    value_ : f64,
    error_ : f64,
}

pub fn PKVar(value : f64, error : f64) -> var {
    var {value_ : value, error_ : error}
}

impl Pow<f64> for var {
    type Output = var;
    fn pow(self, _rhs : f64) -> var {

        if self.value_ == 0. && _rhs < 1.
        {
            return var {value_ : 0., error_ : 1E-310};
        }

        var {
            value_ : (self.value_).powf(_rhs),
            error_ : self.error_*_rhs*(self.value_).powf(_rhs-1.)
        }
    }
}

impl var {
    pub fn getVal(&self) -> f64 {
        self.value_
    }

    pub fn getError(&self) -> f64 {
        self.error_
    }

    pub fn Sqrt(self) -> var {
        Pow::pow(self, 0.5)
    }
}

impl ops::Add<var> for var {
    type Output = var;

    fn add(self, _rhs : var) -> var {
        var {
            value_ : self.value_ + _rhs.value_,
            error_ : (self.error_.powi(2)+_rhs.error_.powi(2)).powf(0.5)
        }
    }
}

impl ops::Sub<var> for var {
    type Output = var;

    fn sub(self, _rhs : var) -> var {
        var {
            value_ : self.value_ - _rhs.value_,
            error_ : (self.error_.powi(2)+_rhs.error_.powi(2)).powf(0.5)
        }
    }
}

impl ops::Mul<var> for var {
    type Output = var;

    fn mul(self, _rhs : var) -> var {
        var {
            value_ : self.value_ * _rhs.value_,
            error_ : ((self.error_ * _rhs.value_).powi(2)+(self.value_*_rhs.error_).powi(2)).powf(0.5)
        }
    }
}

impl ops::Mul<i32> for var {
    type Output = var;

    fn mul(self, _rhs : i32) -> var {
        let rhs = Box::new(_rhs as f64);
        var {
            value_ : self.value_ * *rhs,
            error_ : self.error_ * *rhs
        }
    }
}

impl ops::Mul<f64> for var {
    type Output = var;

    fn mul(self, _rhs : f64) -> var {
        var {
            value_ : self.value_ * _rhs,
            error_ : self.error_ * _rhs
        }
    }
}

impl ops::Div<var> for var {
    type Output = var;

    fn div(self, _rhs : var) -> var {
        var {
            value_ : self.value_ / _rhs.value_,
            error_ : ((self.error_/_rhs.value_).powi(2)+(self.value_*_rhs.error_).powi(2)*_rhs.value_.powi(-4)).powf(0.5)
        }
    }
}

impl ops::Div<i32> for var {
    type Output = var;

    fn div(self, _rhs : i32) -> var {
        let rhs = Box::new(_rhs as f64);
        var {
            value_ : self.value_ / *rhs,
            error_ : self.error_ / *rhs
        }
    }
}

impl ops::Div<f64> for var {
    type Output = var;

    fn div(self, _rhs : f64) -> var {
        var {
            value_ : self.value_ / _rhs,
            error_ : self.error_ / _rhs
        }
    }
}

impl fmt::Display for var {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PKVar({:.4}, {:.4})", self.value_, self.error_)
    }
}

impl Pow<i32> for var {
    type Output = var;
    fn pow(self, _rhs : i32) -> var {

        let rhs = _rhs as f64;

        if self.value_ == 0. && _rhs < 1
        {
            return var {value_ : 0., error_ : 1E-310};
        }

        var {
            value_ : (self.value_).powi(_rhs),
            error_ : self.error_*rhs*(self.value_).powi(_rhs-1)
        }
    }
}