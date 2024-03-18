#ifndef THREADS
#define THREADS

#include <pthread.h>
#include <unistd.h>
#include <stdlib.h>

#define LEN_TASKS 128
#define MAX_THREAD_POOL LEN_TASKS

void* thread_main(void *);

int new_task(int);

int create_thread_pool(int);

int delete_thread_pool(int);

#endif
