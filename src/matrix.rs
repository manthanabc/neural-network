extern crate rand ;

#[derive(Debug)]
pub struct Matrix {
	pub arr : Vec<Vec<f32>>
}

impl Matrix {

	pub fn from_random(colomns: i32, rows: i32) -> Matrix {
		let mut arr = vec![];

		for col in 0..colomns {
			let mut colomn = vec![];
			for row in 0..rows {
				colomn.push(rand::random());
			}
			arr.push(colomn);
		}

		Matrix { arr }
	}

	pub fn from_zeros(colomns: i32, rows: i32) -> Matrix {
		let mut arr = vec![];

		for col in 0..colomns {
			let mut colomn = vec![];
			for row in 0..rows {
				colomn.push(0.0);
			}
			arr.push(colomn);
		}

		Matrix { arr }
	}

	pub fn add(&self, other: &Matrix) -> Matrix {

		if self.arr.len() != other.arr.len() {
			panic!("incompatable matrises");
		}

		let mut result:Matrix = Matrix {
			arr: self.arr.clone()
		};

		for colomn in 0..self.arr.len() {
			let mut temp_colom=self.arr[colomn].clone();
			for element in 0..self.arr[colomn].len() {
				temp_colom[element] = other.arr[colomn][element] + self.arr[colomn][element];
			}
			result.arr[colomn] = temp_colom;
		}
		result
	}

	// here we get a colomn and a row and returns the resulting number
	pub fn one_dimension_multiply<'a>(&self ,other_matrix:&'a Vec<Vec<f32>>, colomn_number: usize, colomn:&Vec<f32>) -> Result<f32, &'a str>
	{
		if other_matrix.len() != colomn.len() {
			return Err("incompatable Matrix")
		}

		let mut result: f32 = 0.0 ;

		for re in 0..colomn.len() {
			result += colomn[re] + other_matrix[re][colomn_number];
		}

		Ok(result)
	}

	pub fn mul(&self, other: &Matrix) -> Matrix {

		if self.arr[0].len() != other.arr.len() {
			panic!("incompatable matrises");
		}

		let mut nv: Vec<Vec<f32>> = self.arr.clone();

		for row in 0..self.arr.len() {
			nv[row] = other.arr[0].clone();
		}

		for colomn in 0..self.arr.len() {
			for el in 0..other.arr[colomn].len() {
				nv[colomn][el] = self.one_dimension_multiply(&other.arr, el, &self.arr[colomn]).unwrap();
			}
		}

		Matrix {
			arr : nv
		}
	}


	pub fn mul_const(self, num:f32) -> Matrix {
		let mut arr = self.arr.clone();

		for colomn in 0..self.arr.len() {
			for row in 0..self.arr[colomn].len() {
				arr[colomn][row] = self.arr[colomn][row] * num ;
			}
		}

		Matrix { arr }
	}

	pub fn transpose(self) -> Matrix {

		let mut arr: Vec<Vec<f32>> = vec![];

		for row in 0..self.arr[0].len() {
			let mut col = vec![];
			for colomn in 0..self.arr.len() {
				col.push( self.arr[colomn][row] );		
			}
			arr.push(col);
		}
		Matrix { arr }
	}
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    	let a = Matrix {arr:vec![vec![2.0,2.0],vec![2.0,2.0]]};
		let b = Matrix {arr:vec![vec![2.0,2.0],vec![2.0,2.0]]};
		let c = a.add(&b);
	
		print!("{:?}",c);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn not_add_diffrensize() {
    	let a = Matrix { arr:vec![vec![2.0,2.0],vec![2.0,2.0]] };
		let b = Matrix { arr:vec![vec![2.0]] };
		let c = a.add(&b);
	
		print!("{:?}",c);
    }

    #[test]
    fn multiplication() {
    	let a = Matrix {arr:vec![vec![2.0,2.0],vec![2.0,2.0]]};
		let b = Matrix {arr:vec![vec![2.0,2.0],vec![2.0,2.0]]};
		let c = a.mul(&b);
	
		let expected = Matrix{arr:vec![vec![8.0,8.0],vec![8.0,8.0]]};

        assert_eq!(expected.arr, c.arr);
    }

    #[test]
    fn multiplication_diffrent_dim() {
    	let a = Matrix {arr:vec![vec![2.0,2.0]]};
		let b = Matrix {arr:vec![vec![2.0],vec![2.0]]};
		let c = a.mul(&b);
	
		let expected = Matrix{arr:vec![vec![8.0]]};

        assert_eq!(expected.arr, c.arr);
    }

    #[test]
    fn transpose() {
    	let matri = Matrix {
    		arr: vec![vec![1.0,2.0]]
    	};
    	let new_arr = vec![vec![1.0], vec![2.0]];

    	assert_eq!(matri.transpose().arr, new_arr);
     }
}