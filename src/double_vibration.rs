use crate::traits::*;
use ndarray::*;

#[derive(Clone, Copy, Debug)]
pub struct Vibration1 {
    pub k: f64,
    pub m_1: f64,
    pub m_2: f64,
}

impl Default for Vibration1 {
    fn default() -> Self {
        Vibration1 {
            k: 1.0,
            m_1: 2.0,
            m_2: 4.0,
        }
    }
}

impl Vibration1 {
    pub fn new(k: f64, m_1: f64, m_2: f64) -> Self {
        Vibration1 {
            k: k,
            m_1: m_1,
            m_2: m_2,
        }
    }
}

impl ModelSpec for Vibration1 {
    type Scalar = f64;
    type Dim = Ix1;

    fn model_size(&self) -> usize {
        4
    }
}

impl Explicit for Vibration1 {
    fn rhs<'a, S>(&mut self, a: &'a mut ArrayBase<S, Ix1>) -> &'a mut ArrayBase<S, Ix1>
    where
        S: DataMut<Elem = f64>,
    {
        let v_1 = a[0];
        let u_1 = a[1];
        let v_2 = a[2];
        let u_2 = a[3];
        a[0] = -2. * self.k / self.m_1 * u_1 + self.k / self.m_2 * u_2;
        a[1] = v_1;
        a[2] = self.k / self.m_1 * u_1 - 2. * self.k / self.m_2 * u_2;
        a[3] = v_2;
        a
    }
}
