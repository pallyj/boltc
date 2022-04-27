typedef struct {
	char* ptr;
	long len;
} strslice_t;

char strslice_eq(strslice_t one, strslice_t two) {
	if (one.len != two.len) {
		return 0;
	}

	for(long i = 0; i < one.len; i += 1) {
		if (one.ptr[i] != two.ptr[i]) {
			return 0;
		}
	}

	return 1;
}

char strslice_neq(strslice_t one, strslice_t two) {
	if (one.len != two.len) {
		return 1;
	}

	for(long i = 0; i < one.len; i += 1) {
		if (one.ptr[i] == two.ptr[i]) {
			return 0;
		}
	}

	return 1;
}

strslice_t strslice_slice(strslice_t slice, long start, long end) {
	long new_len = end - start;
	char* new_ptr = slice.ptr + start;

	strslice_t new_slice = { .ptr=new_ptr, .len=new_len };

	return new_slice;
}

strslice_t strslice_extend(strslice_t slice, long amount) {
	long new_len = slice.len + amount;
	char* new_ptr = slice.ptr;

	strslice_t new_slice = { .ptr=new_ptr, .len=new_len };

	return new_slice;
}

strslice_t strslice_head_slice(strslice_t slice, long amount) {
	long new_len = slice.len - amount;
	char* new_ptr = slice.ptr + amount;

	strslice_t new_slice = { .ptr=new_ptr, .len=new_len };

	return new_slice;
}