#ifndef PARAMETERS
#define PARAMETERS

#include "threads.h"
#include "logging.h"
#include "threads.h"

#include <stdint.h>
#include <arpa/inet.h>

//here are all the global vars
extern char* config_path;
extern enum LOG_LEVEL log_level;
extern int16_t port;
extern int running;
extern int thread_pool_size;
extern pthread_t* pool;
extern int tasks[LEN_TASKS];
extern pthread_mutex_t mutex_array[LEN_TASKS];
extern int time_to_live;

extern pthread_mutex_t len_mutex;
extern int task_len;

extern char public_ip[INET_ADDRSTRLEN];
extern char private_ip[INET_ADDRSTRLEN];

#endif
