#include <stdio.h>

long factorial(long n) {
	long accumulator = 1;

	for (long i = 1; i <= n; i ++) {
		accumulator *= i;
	}

	return accumulator;
}

int main() {
	for ( long i = 0; i < 100000000; i++ ) {
		long n = i % 20;

		long factorial_n = factorial(n);

		printf("%li\n", factorial_n);
	}
	return 0;
}