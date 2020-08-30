use std::ops;
use std::fmt;
use std::cmp;
use num_traits::Pow;

#[derive(Copy, Clone)]
pub struct comp_var {
    _real    : f64,
    _imagin  : f64,
    _modulus : f64,
    _arg     : f64
}

pub fn PKComplexVar(re : f64, im : f64) -> comp_var {
    let arg : f64 = if re != 0. {(im/re).tan()} else {0.};

    let modulus : f64 = (re.powi(2)+im.powi(2)).powf(0.5);

    comp_var {
        _real : re,
        _imagin : im,
        _modulus : modulus,
        _arg : arg
    }
}

impl comp_var {

    fn _setReal(mut self, value : f64) {
        self._real = value
    }

    fn _setImag(mut self, value : f64) {
        self._imagin = value
    }

    pub fn getRe(&self) -> f64 {
        self._real
    }

    pub fn getIm(&self) -> f64 {
        self._imagin
    }

    pub fn getMod(&self) -> f64 {
        self._modulus
    }

    pub fn getArg(&self) -> f64 {
        self._arg
    }

    pub fn returnString(&self, opt : i32) -> String {
        let mut outstring : String = String::from("");

        match opt {
            0 => {
                if self._real != 0. || self._imagin == 0. {
                    outstring.push_str(&format!("{:.4}", &self._real));
                }
                if self._imagin != 0. {
                    if self._imagin > 0. {
                        outstring.push('+');
                    }
                    outstring.push_str(&format!("{:.4}", &self._imagin));
                    outstring.push('i');
                }
                outstring
            }

            1 => {
                if self._imagin == 0. {
                    outstring.push('1');
                }
                else {
                    outstring.push_str(&format!("{:.4}", &self._modulus));
                    outstring.push_str("*exp(");
                    if self._arg < 0. {
                        outstring.push('-');
                    }
                    outstring.push_str(&format!("{:.4}", &self._arg));
                    outstring.push('i');
                    outstring.push(')');
                }
                outstring
            }

            2 => {
                if self._imagin == 0. {
                    outstring.push('1');
                }
                else {
                    outstring.push_str(&format!("{:.4}", &self._real));
                    outstring.push_str("*cos(");
                    outstring.push_str(&format!("{:.4}", &self._arg));
                    outstring.push(')');
                    if self._imagin > 0. {
                        outstring.push('+');
                    }
                    outstring.push_str(&format!("{:.4}", &self._imagin));
                    outstring.push_str("i*sin(");
                    outstring.push_str(&format!("{:.4}", &self._arg ));
                    outstring.push(')');
                }
                outstring
            }

            _ => {
                panic!("ERROR: Invalid integer argument for PKComplexVar::Print(int) options are 0, 1, 2");
            }
        }
    }
}

impl ops::Add<comp_var> for comp_var {
    type Output = comp_var;

    fn add(self, _rhs : comp_var) -> comp_var {
        PKComplexVar(
            self._real + _rhs._real,
            self._imagin + _rhs._imagin
        )
    }
}

impl ops::Sub<comp_var> for comp_var {
    type Output = comp_var;

    fn sub(self, _rhs : comp_var) -> comp_var {
        PKComplexVar(
            self._real - _rhs._real,
            self._imagin - _rhs._imagin
        )
    }
}

impl ops::Mul<comp_var> for comp_var {
    type Output = comp_var;

    fn mul(self, _rhs : comp_var) -> comp_var {
        PKComplexVar(
            self._real*_rhs._real - self._imagin*_rhs._imagin,
            self._real*_rhs._imagin + self._imagin*_rhs._real
        )
    }
}

impl ops::Mul<i32> for comp_var {
    type Output = comp_var;

    fn mul(self, _rhs : i32) -> comp_var {
        let rhs = Box::new(_rhs as f64);
        PKComplexVar(
            self._real * *rhs,
            self._imagin * *rhs
        )
    }
}

impl ops::Mul<comp_var> for i32 {
    type Output = comp_var;

    fn mul(self, _rhs : comp_var) -> comp_var {
        let self_f = Box::new(self as f64);
        PKComplexVar(
            _rhs._real * *self_f,
            _rhs._imagin * *self_f
        )
    }
}

impl ops::Mul<comp_var> for f64 {
    type Output = comp_var;

    fn mul(self, _rhs : comp_var) -> comp_var {
        PKComplexVar(
            _rhs._real*self,
            _rhs._imagin*self
        )
    }
}

impl ops::Mul<f64> for comp_var {
    type Output = comp_var;

    fn mul(self, _rhs : f64) -> comp_var {
        PKComplexVar(
            self._real*_rhs,
            self._imagin*_rhs
        )
    }
}

impl ops::Div<comp_var> for comp_var {
    type Output = comp_var;

    fn div(self, _rhs : comp_var) -> comp_var {
        let modulus : f64 = self._modulus/_rhs._modulus;
        let arg : f64 = self._arg - _rhs._arg;
        PKComplexVar(
            modulus/((1.+arg.atan()).powf(0.5)),
            (modulus*arg.atan())/((1.+arg.atan().powi(2)).powf(0.5))
        )
    }
}

impl ops::Div<i32> for comp_var {
    type Output = comp_var;

    fn div(self, _rhs : i32) -> comp_var {
        let rhs = Box::new(_rhs as f64);
        PKComplexVar(
            self._real / *rhs,
            self._imagin / *rhs
        )
    }
}

impl ops::Div<f64> for comp_var {
    type Output = comp_var;

    fn div(self, _rhs : f64) -> comp_var {
        PKComplexVar(
            self._real/_rhs,
            self._imagin/_rhs
        )
    }
}

impl cmp::PartialEq for comp_var {
    fn eq(&self, other : &Self) -> bool {
        self._real == other._real && self._imagin == other._imagin
    }
}