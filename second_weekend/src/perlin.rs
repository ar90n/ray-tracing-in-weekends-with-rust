use rand::prelude::*;
use rand::random;

use super::vec3::*;

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                accum += ((i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                    * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                    * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww)))
                    * dot(&c[i][j][k], &weight);
            }
        }
    }
    accum
}

fn generate_perm() -> [u8; 256] {
    let mut tbl = [0u8; 256];
    for i in 0..tbl.len() {
        tbl[i] = i as u8;
    }
    let mut rng = rand::thread_rng();
    tbl.shuffle(&mut rng);
    tbl
}

fn generate() -> [Vec3; 256] {
    let mut tbl = [Vec3::default(); 256];
    for i in 0..tbl.len() {
        tbl[i] = Vec3::new(
            -1.0 + 2.0 * random::<f32>(),
            -1.0 + 2.0 * random::<f32>(),
            -1.0 + 2.0 * random::<f32>(),
        );
    }
    tbl
}

lazy_static::lazy_static! {
    static ref RANFLOAT: [Vec3; 256] =  generate();
    static ref PERM_X: [u8; 256] =  generate_perm();
    static ref PERM_Y: [u8; 256] =  generate_perm();
    static ref PERM_Z: [u8; 256] =  generate_perm();
}

#[derive(Debug, Default)]
pub struct Perlin {}

impl Perlin {
    pub fn noise(&self, p: &Vec3) -> f32 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = RANFLOAT[(PERM_X[((i + di as i32) & 255) as usize]
                        ^ PERM_Y[((j + dj as i32) & 255) as usize]
                        ^ PERM_Z[((k + dk as i32) & 255) as usize])
                        as usize]
                }
            }
        }
        perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Vec3, depth: u32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum
    }
}
