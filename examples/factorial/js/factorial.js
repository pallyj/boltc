function factorial(n) {
	let accumulator = 1

	for (let i = 1; i <= n; i++) {
		accumulator *= i;
	}

	return accumulator;
}

let max = 1000000;

for (let i = 0; i < max; i++) {
	console.log(factorial(i % 20));
}