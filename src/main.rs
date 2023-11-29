use std::ops;

#[derive(Debug)]
struct Vector;
struct Scalar;

// impl Vector { pub fn new() -> Self {Vector} }
// impl std::fmt::Display for Vector {
// 	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "{}",self.basis)
// 	}
// }

impl ops::Add<Vector> for Vector {
	type Output = Vector;
	fn add(self, _rhs: Vector) -> Vector{
		// println!("Adding Vectors");
		Vector
	}
}
impl ops::Mul<Scalar> for Vector{
	type Output = Vector;
	fn mul(self, _rhs: Scalar) -> Vector{
		// println!("Multiplying Vectors and Scalars");
		Vector
	}
}
impl ops::Mul<Vector> for Scalar{
	type Output = Vector;
	fn mul(self, _rhs: Vector) -> Vector {
		// println!("Multiplying Vectors and Scalars");
		Vector
	}
}

fn main() {
	let u: Vector = Vector;
	let v: Vector = Vector;
	// let s: Scalar = Scalar;
	println!("u+v = {:?}",u+v);
}
