#include <stdio.h>

unsigned long factorial(unsigned long);

int main() {
	for (int i = 0; i < 20; i += 1) {
		printf("%lu\n", factorial(i));
	}
}