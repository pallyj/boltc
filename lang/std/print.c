#include <stdio.h>

void printInt(long i) {
	printf("%li", i);
}

void printUInt(unsigned long i) {
	printf("%lu", i);
}

void printInt8(char i) {
	printf("%hhi", i);
}

void printUInt8(unsigned char i) {
	printf("%hhu", i);
}

void printInt16(short i) {
	printf("%hi", i);
}

void printUInt16(unsigned short i) {
	printf("%hu", i);
}

void printInt32(int i) {
	printf("%i", i);
}

void printUInt32(unsigned int i) {
	printf("%u", i);
}

void printInt64(long i) {
	printf("%li", i);
}

void printUInt64(unsigned long i) {
	printf("%lu", i);
}

void printDouble(double n) {
	printf("%f", n);
}

void printFloat(float n) {
	printf("%f", n);
}

void printBool(unsigned char b) {
	printf(b ? "true" : "false");
}

typedef struct {
	char*		  string;
	unsigned long len;
} strslice_t;

void printString(strslice_t slice) {
	printf("%.*s", slice.len, slice.string);
}

void printLine() {
	printf("\n");
}