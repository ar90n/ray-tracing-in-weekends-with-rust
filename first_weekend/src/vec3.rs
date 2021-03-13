use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3([f32; 3]);
impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self([e0, e1, e2])
    }
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn r(&self) -> f32 {
        self.0[0]
    }

    pub fn g(&self) -> f32 {
        self.0[1]
    }

    pub fn b(&self) -> f32 {
        self.0[2]
    }

    pub fn squared_length(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
    pub fn lenght(&self) -> f32 {
        self.squared_length().sqrt()
    }
    pub fn make_unit_vector(&self) -> Self {
        let k = 1.0 / self.lenght();
        Self::new(self.x() / k, self.y() / k, self.z() / k)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0[0] -= rhs.0[0];
        self.0[1] -= rhs.0[1];
        self.0[2] -= rhs.0[2];
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0[0] *= rhs.0[0];
        self.0[1] *= rhs.0[1];
        self.0[2] *= rhs.0[2];
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.0[0] /= rhs.0[0];
        self.0[1] /= rhs.0[1];
        self.0[2] /= rhs.0[2];
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Self::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Self::new(
            self.x() / other.x(),
            self.y() / other.y(),
            self.z() / other.z(),
        )
    }
}

#[test]
fn test_nen() {
    let vec = Vec3::new(0.0, 1.0, 2.0);
    let n_vec = -vec;
    assert_eq!(n_vec.x(), 0.0);
    assert_eq!(n_vec.y(), -1.0);
    assert_eq!(n_vec.z(), -2.0);
}

fn test_index() {
    let vec = Vec3::new(0.0, 1.0, 2.0);
    assert_eq!(vec[0], 0.0);
    assert_eq!(vec[1], -1.0);
    assert_eq!(vec[2], -2.0);
}

fn test_add_assign() {
    let mut vec = Vec3::new(0.0, 1.0, 2.0);
    vec += Vec3::new(3.0, 4.0, 5.0);
    assert_eq!(vec.x(), 3.0);
    assert_eq!(vec.y(), 5.0);
    assert_eq!(vec.z(), 7.0);
}

fn test_sub_assign() {
    let mut vec = Vec3::new(0.0, 1.0, 2.0);
    vec -= Vec3::new(3.0, 4.0, 5.0);
    assert_eq!(vec.x(), -3.0);
    assert_eq!(vec.y(), -3.0);
    assert_eq!(vec.z(), -3.0);
}

fn test_mul_assign() {
    let mut vec = Vec3::new(0.0, 1.0, 2.0);
    vec *= Vec3::new(3.0, 4.0, 5.0);
    assert_eq!(vec.x(), 0.0);
    assert_eq!(vec.y(), 4.0);
    assert_eq!(vec.z(), 10.0);
}

fn test_div_assign() {
    let mut vec = Vec3::new(0.0, 1.0, 2.0);
    vec /= Vec3::new(3.0, 4.0, 5.0);
    assert_eq!(vec.x(), 0.0);
    assert_eq!(vec.y(), 0.25);
    assert_eq!(vec.z(), 0.4);
}

fn test_add() {
    let mut vec = Vec3::new(0.0, 1.0, 2.0) + Vec3::new(3.0, 4.0, 5.0);
    assert_eq!(vec.x(), 3.0);
    assert_eq!(vec.y(), 5.0);
    assert_eq!(vec.z(), 7.0);
}

fn test_sub() {
    let mut vec = Vec3::new(0.0, 1.0, 2.0) - Vec3::new(3.0, 4.0, 5.0);
    assert_eq!(vec.x(), -3.0);
    assert_eq!(vec.y(), -3.0);
    assert_eq!(vec.z(), -3.0);
}

fn test_mul() {
    let mut vec = Vec3::new(0.0, 1.0, 2.0) * Vec3::new(3.0, 4.0, 5.0);
    assert_eq!(vec.x(), 0.0);
    assert_eq!(vec.y(), 4.0);
    assert_eq!(vec.z(), 10.0);
}

fn test_div() {
    let mut vec = Vec3::new(0.0, 1.0, 2.0) / Vec3::new(3.0, 4.0, 5.0);
    assert_eq!(vec.x(), 0.0);
    assert_eq!(vec.y(), 0.25);
    assert_eq!(vec.z(), 0.4);
}
