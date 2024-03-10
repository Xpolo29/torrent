#include "database.h"

enum request_t { announce = 0, look, getfile, update };

enum op_t { eq, gt, lt };

int filter(struct data *list, char *filename, long filesize, enum op_t op);

int load_config(char *filepath);
