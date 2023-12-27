use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Display;

fn main() {
    let input = include_str!("test.txt");

    let hailstones : Vec<Hailstone> = input.split("\n")
        .map(str::trim)
        .map(|line| line.into())
        .collect();

    let amt = find_crossings(&hailstones, 
        200000000000000.0, 400000000000000.0,
        false
    );

    println!("PART 1: {}", amt);
}

fn find_crossings(hailstones : &Vec<Hailstone>, min : f64, max : f64, debug : bool) -> usize {
    let mut sum = 0;

    for i in 0..hailstones.len() {
        for j in i+1..hailstones.len() {
            if debug {
                println!("Hailstone A: {}", hailstones[i]);
                println!("Hailstone B: {}", hailstones[j]);
            };
            

            match crossing(&hailstones[i], &hailstones[j], debug) {
                Some((x, y)) => {
                    if x < min || x > max || y < min || y > max {
                        if debug {
                            println!("Hailstones' paths will cross outside the test area (at x={}, y={}).", x, y);
                        }
                    } else {
                        if debug {
                            println!("Hailstones' paths will cross inside the test area (at x={}, y={}).", x, y);
                        }
                        sum += 1;
                    }
                },
                None => {},
            }

            if debug {
                println!();
            }
        }
    }

    sum
}

fn part2(hailstones : &Vec<Hailstone>) {
    // p1 + v1 * t1 = v? * t1
    // p2 + v2 * t2 = v? * t2
    // p3 + v3 * t3 = v? * t3

    // v? = p1 / t1 + v1
    // v? = p2 / t2 + v2
    // v? = p3 / t3 + v3

    // t1 = p1 / (v? - v1)
    // t2 = p2 / (v? - v2)
    // t3 = p3 / (v? - v3)
}

#[derive(Debug)]
struct Hailstone {
    position : Vec3,
    velocity : Vec3
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}, @ {}, {}, {}", 
            self.position.x, self.position.y, self.position.z, 
            self.velocity.x, self.velocity.y, self.velocity.z
        )
    }
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let parts : Vec<&str> = value.split(" @ ").collect();
        Self { position: parts[0].into(), velocity: parts[1].into() }
    }
}

fn crossing(a : &Hailstone, b : &Hailstone, debug : bool) -> Option<(f64, f64)> {
    // a.position + t1 * a.velocity = b.position + t2 * b.velocity
    // => t1 = (b.position + t2 * b.velocity - a.position) / a.velocity

    // => t1 = (b.p.x + t2 * b.v.x - a.p.x) / a.v.x
    //    t1 = (b.p.y + t2 * b.v.y - a.p.y) / a.v.y
    
    // => (b.p.x + t2 * b.v.x - a.p.x) / a.v.x = (b.p.y + t2 * b.v.y - a.p.y) / a.v.y

    if a.velocity.is_parallel(b.velocity) {
        if debug {
            println!("Hailstones' paths are parallel; they never intersect.");
        }
        return None;
    }
    

    let div = a.velocity.x - b.velocity.x * a.velocity.y / b.velocity.y;

    if div.approx_eq(0.0) {
        return None;
    }

    let upper = (a.position.y - b.position.y) * b.velocity.x / b.velocity.y - (a.position.x - b.position.x);

    let t_a = upper / div;

    let point = a.position + a.velocity * t_a;

    let t_b = (point - b.position).div_partwise(b.velocity).x;

    if t_a < 0.0 && t_b < 0.0 {
        if debug {
            println!("Hailstones' paths crossed in the past for both hailstones.");
        }
        return None;
    } else if t_a < 0.0 {
        if debug {
            println!("Hailstones' paths crossed in the past for hailstone A.");
        }
        return None;
    } else if t_b < 0.0 {
        if debug {
            println!("Hailstones' paths crossed in the past for hailstone B.");
        }
        return None;
    }

    return Some((point.x, point.y))
}

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x : f64,
    y : f64,
    z : f64,
}

impl Vec3 {
    fn div_partwise(self, other: Vec3) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }

    fn is_parallel(self, other : Vec3) -> bool {
        let x_ratio = self.x / other.x;
        let y_ratio = self.y / other.y;
        let z_ratio = self.z / other.z;

        x_ratio.approx_eq(y_ratio) && x_ratio.approx_eq(z_ratio) && y_ratio.approx_eq(z_ratio)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl From<&str> for Vec3 {
    fn from(value: &str) -> Self {
        let parts : Vec<&str> = value.split(", ").collect();
        Self { 
            x: parts[0].trim().parse().expect(&format!("{} don't parse", parts[0])), 
            y: parts[1].trim().parse().expect(&format!("{} don't parse", parts[1])), 
            z: parts[2].trim().parse().expect(&format!("{} don't parse", parts[2])), 
        }
    }
}

trait ApproxEq {
    fn approx_eq(self, other : Self) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(self, other : Self) -> bool {
        (self - other).abs() < 0.01
    }
}