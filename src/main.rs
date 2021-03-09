mod matrix;
use matrix::Matrix;

fn main() {
	let a = Matrix {arr:vec![vec![2.0,2.0],vec![2.0,2.0]]};
	let b = Matrix::from_random(2,2);
	let c = a.add(&b);
	
	print!("{:?}",c);
}

// fn main() {
	// let layout = [3, 4, 5];
	// let nn = neural_network::new(layout);

	// let options = config {
	// 	learning_rate: 0.2
	// }

	// nn.config(options);

	// for (input, otput) in examples {
	// 	nn.train(input, output);
	// }

	// let predicted = nn.predict(input);
// }