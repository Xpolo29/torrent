#include "database.h"
#include <sys/socket.h>
#include <netinet/in.h>
#define LEN_ARGS 10
#define LEN_TASKS 128
#define MAX_THREAD_POOL LEN_TASKS
enum request_t { announce = 0, look, getfile, update };

enum op_t { eq, gt, lt };

int filter(struct data *list, char *filename, long filesize, enum op_t op);

int load_config(char *filepath);

static const char ARGS[LEN_ARGS][16] = {
	"-v", "--verbose",
       	"-h", "--help",
       	"-p", "--port",
       	"-c", "--config",
	"-m", "--max-conn"
};

enum LOG_LEVEL{
	ERROR=0, WARNING, LOG, DEBUG, NONE
};

char* log_level_to_string(enum LOG_LEVEL);

void logging(enum LOG_LEVEL, const char*, ...);

char* parse_request(char* request, int peer_ip);

int listen_on(int sock);

int create_master_sock(int);

int send_msg(int socket);

void sigint_handler(int); 

int process(int);

int create_thread_pool(int);

void* thread_main(void*);

int new_task(int);
	
int delete_thread_pool();
