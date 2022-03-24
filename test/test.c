#include <stdio.h>

typedef struct {
	long repr;
} Int64;

unsigned long printi(unsigned long i) {
	printf("%ld\n", i);
	return 0;
}