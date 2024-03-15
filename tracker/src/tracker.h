#include "database.h"
#include <sys/socket.h>
#include <netinet/in.h>
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

enum LOG_LEVEL{
	ERROR=0, WARNING, LOG, NONE
};

char* log_level_to_string(enum LOG_LEVEL);

void logging(enum LOG_LEVEL, const char*, ...);

char* parse_request(char* request, int peer_ip);

int listen_on(int sock);

int create_master_sock(int);

int send_msg(int socket);

void sigint_handler(int); 

int process(int);
