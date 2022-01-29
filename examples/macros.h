#define SECRET 1 \

#define ADD(a, b) (a + b) \

#define SLICE(T) struct {
	int len;
	T* ptr;
} \

SLICE(int) get_slice() {

}