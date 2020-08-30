use std::ops;
use std::fmt;
use num_traits::Pow;

#[derive(Copy, Clone)]
pub struct var {
    _value : f64,
    _error : f64,
}

pub fn PKVar(value : f64, error : f64) -> var {
    var {_value : value, _error : error}
}

impl Pow<f64> for var {
    type Output = var;
    fn pow(self, _rhs : f64) -> var {

        if self._value == 0. && _rhs < 1.
        {
            return var {_value : 0., _error : 1E-310};
        }

        var {
            _value : (self._value).powf(_rhs),
            _error : self._error*_rhs*(self._value).powf(_rhs-1.)
        }
    }
}

impl var {
    pub fn getVal(&self) -> f64 {
        self._value
    }

    pub fn getError(&self) -> f64 {
        self._error
    }

    pub fn Sqrt(self) -> var {
        Pow::pow(self, 0.5)
    }
}

impl ops::Add<var> for var {
    type Output = var;

    fn add(self, _rhs : var) -> var {
        var {
            _value : self._value + _rhs._value,
            _error : (self._error.powi(2)+_rhs._error.powi(2)).powf(0.5)
        }
    }
}

impl ops::Sub<var> for var {
    type Output = var;

    fn sub(self, _rhs : var) -> var {
        var {
            _value : self._value - _rhs._value,
            _error : (self._error.powi(2)+_rhs._error.powi(2)).powf(0.5)
        }
    }
}

impl ops::Mul<var> for var {
    type Output = var;

    fn mul(self, _rhs : var) -> var {
        var {
            _value : self._value * _rhs._value,
            _error : ((self._error * _rhs._value).powi(2)+(self._value*_rhs._error).powi(2)).powf(0.5)
        }
    }
}

impl ops::Mul<i32> for var {
    type Output = var;

    fn mul(self, _rhs : i32) -> var {
        let rhs = Box::new(_rhs as f64);
        var {
            _value : self._value * *rhs,
            _error : self._error * *rhs
        }
    }
}

impl ops::Mul<f64> for var {
    type Output = var;

    fn mul(self, _rhs : f64) -> var {
        var {
            _value : self._value * _rhs,
            _error : self._error * _rhs
        }
    }
}

impl ops::Div<var> for var {
    type Output = var;

    fn div(self, _rhs : var) -> var {
        var {
            _value : self._value / _rhs._value,
            _error : ((self._error/_rhs._value).powi(2)+(self._value*_rhs._error).powi(2)*_rhs._value.powi(-4)).powf(0.5)
        }
    }
}

impl ops::Div<i32> for var {
    type Output = var;

    fn div(self, _rhs : i32) -> var {
        let rhs = Box::new(_rhs as f64);
        var {
            _value : self._value / *rhs,
            _error : self._error / *rhs
        }
    }
}

impl ops::Div<f64> for var {
    type Output = var;

    fn div(self, _rhs : f64) -> var {
        var {
            _value : self._value / _rhs,
            _error : self._error / _rhs
        }
    }
}

impl fmt::Display for var {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PKVar({:.4}, {:.4})", self._value, self._error)
    }
}

impl Pow<i32> for var {
    type Output = var;
    fn pow(self, _rhs : i32) -> var {

        let rhs = _rhs as f64;

        if self._value == 0. && _rhs < 1
        {
            return var {_value : 0., _error : 1E-310};
        }

        var {
            _value : (self._value).powi(_rhs),
            _error : self._error*rhs*(self._value).powi(_rhs-1)
        }
    }
}