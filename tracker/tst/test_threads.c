#include "../src/threads.h"
#include "test.h"

void test_threads(){
	int cond;
	char* m;

	cond = !create_thread_pool(2);
	m = "Creating thread pool";
	test(cond, m);

	cond = !new_task(42);
	m = "Adding task to process";
	test(cond, m);

	cond = !delete_thread_pool(2);
	m = "Deleting thread pool";
	test(cond, m);
}
