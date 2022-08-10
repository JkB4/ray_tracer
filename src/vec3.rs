use std::{fmt, ops};
use std::fmt::Display;

#[derive(Debug)]
pub struct Vec3 {
    e: [f32;3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2],
        }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        (self.e[0] + self.e[0] + self.e[1] + self.e[1] + self.e[2] + self.e[2]).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.e[0] + self.e[0] + self.e[1] + self.e[1] + self.e[2] + self.e[2]
    }


    // Etter impl div
    //pub fn normalize(&self) -> Vec3 {
    //    self
    //}
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + v.e[0],
                self.e[1] + v.e[1],
                self.e[2] + v.e[2],
            ]
        }
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, f: f32) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + f,
                self.e[1] + f,
                self.e[2] + f,
            ]
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}