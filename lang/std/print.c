#include <stdio.h>
#include <unistd.h>

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
	printf("%.*s", (int)slice.len, slice.string);
}

void printChar(int c) {
	printf("%lc", c);
}

void printLine() {
	printf("\n");
}

long readInternalInt() {
	char buffer[20] = { 0 };
	size_t len = 20;
	read(0, buffer, 20);
	long integerRead;

	sscanf(buffer, "%ld", &integerRead);

	return integerRead;
}

unsigned long readInternalUInt() {
	char buffer[20];
	read(0, buffer, 20);
	unsigned long integerRead;

	sscanf(buffer, "%lu", &integerRead);

	return integerRead;
}

unsigned int readInternalChar() {
	char c = 0;

	read(0, &c, 1);

	return c;
}
