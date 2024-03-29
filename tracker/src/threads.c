#include "threads.h"
#include "logging.h"
#include "parameters.h"
#include "tracker.h"

//Main thread fonction, permanently looking for task to process
void* thread_main(void* arg){
	int treated = 0;
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
				pthread_mutex_lock(&len_mutex);
				task_len--;
				pthread_mutex_unlock(&len_mutex);

				process(temp);
				treated++;
			}
			else
				pthread_mutex_unlock(&mutex_array[i]);
		}
		mysleep();
		i = (i + 1) % LEN_TASKS;

	}	
	logging(DEBUG, "Thread %lu stopped and processed %d tasks\n", pthread_self(), treated);
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

			pthread_mutex_lock(&len_mutex);
			task_len++;
			pthread_mutex_unlock(&len_mutex);

			return 0;
		}
		pthread_mutex_unlock(&mutex_array[i]);
	}	

	logging(WARNING, "Task list is full\n");
	return 1;
}

//create thread pool that will be processing tasks
int create_thread_pool(int size){
	int res = 0;
	logging(LOG, "Creating thread pool of size %d\n", size);
	pool = malloc(sizeof(pthread_t) * size);
	for(int i = 0; i < size; ++i){
		res += pthread_create(&pool[i], NULL, thread_main, NULL);
		res += pthread_mutex_init(&mutex_array[i], NULL);
	}	
	res += pthread_mutex_init(&len_mutex, NULL);
	if(res > 0)logging(ERROR, "Could not create thread pool\n");
	return res;
}

//delete thread pool, cleanup before exit
int delete_thread_pool(int size){
	logging(LOG, "Deleting thread pool of size %d\n", size);
	running = 0;
	int res = 0;
	for(int i = 0; i < size; ++i){
		res += pthread_join(pool[i], NULL);
	}
	if(res)logging(WARNING, "Could not stop threads cleanly");
	if(!pool)return 1;
	free(pool);
	return 0;
}

