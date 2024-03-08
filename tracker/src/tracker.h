enum request_t {
	announce=0, look, getfile, update
};

enum op_t {
	eq, gt, lt
};

int check_md5(char* filename, char hash[64]);

struct data* filter(char* filename, double filesize, enum op_t op);
