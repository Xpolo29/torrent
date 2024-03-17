#include "threads.h"
#include "logging.h"
#include "parameters.h"
#include "tracker.h"

void* thread_main(void* arg){
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

int create_thread_pool(int size){
	logging(LOG, "Creating thread pool of size %d\n", thread_pool_size);
	pool = malloc(sizeof(pthread_t) * thread_pool_size);
	for(int i = 0; i < thread_pool_size; ++i){
		pthread_create(&pool[i], NULL, thread_main, NULL);
	}	
	return 0;
}

int delete_thread_pool(){
	logging(LOG, "Deleting thread pool of size %d\n", thread_pool_size);
	for(int i = 0; i < thread_pool_size; ++i){
		pthread_join(pool[i], NULL);
	}
	if(!pool)return 1;
	free(pool);
	return 0;
}

