#include <stdio.h>

typedef struct {
	long repr;
} Int64;

unsigned long printi(Int64 i) {
	printf("%ld\n", i.repr);
	return 0;
}

unsigned long printiraw(long i) {
	printf("%ld\n", i);
	return 0;
}