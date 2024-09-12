#ifndef THREADS
#define THREADS

#include <pthread.h>
#include <unistd.h>
#include <stdlib.h>

#define LEN_TASKS 128
#define MAX_THREAD_POOL LEN_TASKS

//thread main fonction that wait for new tasks to be processed
void* thread_main(void *);

//add new task to be processed by threads 
int new_task(int);

//create thread pool of size <int>
int create_thread_pool(int);

//delete previously created thread pool, need creating size as <int>
int delete_thread_pool(int);

#endif
