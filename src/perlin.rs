use crate::{
    utils,
    vec3::{Point3, Vec3},
};

const POINT_COUNT: i32 = 256;

pub struct Perlin {
    rand_vec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn noise(&self, point: &Point3) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let idx_i = utils::float_to_int_truncate(point.x().floor());
        let idx_j = utils::float_to_int_truncate(point.y().floor());
        let idx_k = utils::float_to_int_truncate(point.z().floor());

        let mut c: Vec<Vec<Vec<Vec3>>> = Vec::with_capacity(2);
        for di in 0..2 {
            let mut vec_i = Vec::with_capacity(2);
            for dj in 0..2 {
                let mut vec_j = Vec::with_capacity(2);
                for dk in 0..2 {
                    #[allow(clippy::cast_sign_loss)] // Its safe
                    let index = (self.perm_x[((idx_i + di) & 255) as usize]
                        ^ self.perm_y[((idx_j + dj) & 255) as usize]
                        ^ self.perm_z[((idx_k + dk) & 255) as usize])
                        as usize;
                    vec_j.push(self.rand_vec[index]);
                }
                vec_i.push(vec_j);
            }
            c.push(vec_i);
        }

        perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut acc = 0.0;
        let mut p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(&p);
            weight *= 0.5;
            p *= 2.0;
        }

        acc.abs()
    }
}

impl Default for Perlin {
    fn default() -> Perlin {
        let mut rand_vec = Vec::with_capacity(POINT_COUNT as usize);
        for _ in 0..POINT_COUNT {
            rand_vec.push(Vec3::random_range(-1.0, 1.0).unit_vector());
        }

        Perlin {
            rand_vec,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }
}

fn perlin_generate_perm() -> Vec<i32> {
    let mut perm = Vec::with_capacity(POINT_COUNT as usize);
    perm.extend(0..POINT_COUNT);

    for i in (1..POINT_COUNT).rev() {
        let target = utils::random_int(0, i);
        perm.swap(i.try_into().unwrap(), target.try_into().unwrap());
    }

    perm
}

fn perlin_interp(c: &[Vec<Vec<Vec3>>], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut acc = 0.0;

    for (i, vec_i) in c.iter().enumerate() {
        for (j, vec_j) in vec_i.iter().enumerate() {
            for (k, value) in vec_j.iter().enumerate() {
                #[allow(clippy::cast_precision_loss)] // Its safe
                let (fi, fj, fk) = (i as f64, j as f64, k as f64);
                let weight = Vec3::new(u - fi, v - fj, w - fk);
                acc += (fi * uu + (1.0 - fi) * (1.0 - uu))
                    * (fj * vv + (1.0 - fj) * (1.0 - vv))
                    * (fk * ww + (1.0 - fk) * (1.0 - ww))
                    * value.dot(&weight);
            }
        }
    }

    acc
}
