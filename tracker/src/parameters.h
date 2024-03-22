#ifndef PARAMETERS
#define PARAMETERS

#include "threads.h"
#include "logging.h"
#include "threads.h"

#include <stdint.h>

extern char* config_path;
extern enum LOG_LEVEL log_level;
extern int16_t port;
extern int running;
extern int thread_pool_size;
extern pthread_t* pool;
extern int tasks[LEN_TASKS];
extern pthread_mutex_t mutex_array[LEN_TASKS];

#endif
