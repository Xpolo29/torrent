#include "database.h"

enum request_t { announce = 0, look, getfile, update };

enum op_t { eq, gt, lt };

int check_md5(char *filename, char hash[64]);

int filter(struct data *list, char *filename, double filesize, enum op_t op);

int load_config(char *filepath);
