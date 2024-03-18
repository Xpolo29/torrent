#include "threads.h"
#include "logging.h"
#include "parameters.h"
#include "tracker.h"

//Main thread fonction, permanently looking for task to process
void* thread_main(void* arg){
	(void)arg; // to disable warning
	logging(DEBUG, "Thread %lu started\n", pthread_self());
	int i = 0;
	while(running){
		int used = pthread_mutex_trylock(&mutex_array[i]);
		if(!used){
			if(tasks[i]){
				int temp = tasks[i];
				tasks[i] = 0;
				pthread_mutex_unlock(&mutex_array[i]);
				logging(DEBUG, "Thread %lu processing task %d\n", pthread_self(), temp);
				process(temp);
			}
			pthread_mutex_unlock(&mutex_array[i]);
		}

		i = (i + 1) % LEN_TASKS;
		usleep(1000);
	}	
	logging(DEBUG, "Thread %lu stopped\n", pthread_self());
	return 0;
}

//add new task to be processed
int new_task(int conn){
	logging(DEBUG, "Adding new task to handle conn=%d\n", conn);

	for(int i = 0; i < LEN_TASKS; ++i){
		pthread_mutex_lock(&mutex_array[i]);
		if(!tasks[i]){
			tasks[i] = conn;
			pthread_mutex_unlock(&mutex_array[i]);
			return 0;
		}
		pthread_mutex_unlock(&mutex_array[i]);
	}	

	logging(WARNING, "Task list is full\n");
	return 1;
}

//create thread pool that will be processing tasks
int create_thread_pool(int size){
	logging(LOG, "Creating thread pool of size %d\n", size);
	pool = malloc(sizeof(pthread_t) * size);
	for(int i = 0; i < size; ++i){
		pthread_create(&pool[i], NULL, thread_main, NULL);
	}	
	return 0;
}

//delete thread pool, cleanup before exit
int delete_thread_pool(){
	logging(LOG, "Deleting thread pool of size %d\n", thread_pool_size);
	int res = 0;
	for(int i = 0; i < thread_pool_size; ++i){
		res += pthread_join(pool[i], NULL);
	}
	if(res)logging(WARNING, "Could not stop threads cleanly");
	if(!pool)return 1;
	free(pool);
	return 0;
}

