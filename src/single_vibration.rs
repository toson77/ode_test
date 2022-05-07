use crate::traits::*;
use ndarray::*;

#[derive(Clone, Copy, Debug)]
pub struct Vibration1 {
    pub k: f64,
    pub m: f64,
}

impl Default for Vibration1 {
    fn default() -> Self {
        Vibration1 { k: 1.0, m: 2.0 }
    }
}

impl Vibration1 {
    pub fn new(k: f64, m: f64) -> Self {
        Vibration1 { k: k, m: m }
    }
}

impl ModelSpec for Vibration1 {
    type Scalar = f64;
    type Dim = Ix1;

    fn model_size(&self) -> usize {
        2
    }
}

impl Explicit for Vibration1 {
    fn rhs<'a, S>(&mut self, a: &'a mut ArrayBase<S, Ix1>) -> &'a mut ArrayBase<S, Ix1>
    where
        S: DataMut<Elem = f64>,
    {
        let v = a[0];
        let u = a[1];
        a[0] = -self.k / self.m * u;
        a[1] = v;
        a
    }
}
