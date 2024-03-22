#include "parameters.h"

//global var base value. Default values are loaded from config
char* config_path = "config.ini";
enum LOG_LEVEL log_level = WARNING;
int16_t port = -1;
int running = 1;
int thread_pool_size = -1;
pthread_t* pool;
int tasks[LEN_TASKS];
pthread_mutex_t mutex_array[LEN_TASKS];


