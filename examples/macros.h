#define SECRET 1 \

#define ADD(a, b) (a + b) \

#define SLICE(T) struct {
	int len;
	T* ptr;
} \


struct X {
	SLICE(int)
}