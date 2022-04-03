#include <stdio.h>

void printInt(long i) {
	printf("%li\n", i);
}

void printUInt(unsigned long i) {
	printf("%lu\n", i);
}

void printInt8(char i) {
	printf("%hhi\n", i);
}

void printUInt8(unsigned char i) {
	printf("%hhu\n", i);
}

void printInt16(short i) {
	printf("%hi\n", i);
}

void printUInt16(unsigned short i) {
	printf("%hu\n", i);
}

void printInt32(int i) {
	printf("%i\n", i);
}

void printUInt32(unsigned int i) {
	printf("%u\n", i);
}

void printInt64(long i) {
	printf("%li\n", i);
}

void printUInt64(unsigned long i) {
	printf("%lu\n", i);
}

void printDouble(double n) {
	printf("%f\n", n);
}

void printFloat(float n) {
	printf("%f\n", n);
}

void printBool(unsigned char b) {
	printf(b ? "true" : "false");
}