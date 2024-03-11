#include "database.h"
#define LEN_ARGS 8
enum request_t { announce = 0, look, getfile, update };

enum op_t { eq, gt, lt };

int filter(struct data *list, char *filename, long filesize, enum op_t op);

int load_config(char *filepath);

static const char ARGS[LEN_ARGS][16] = {
	"-v", "--verbose",
       	"-h", "--help",
       	"-p", "--port",
       	"-c", "--config" 
};
