#include "../src/tracker.h"
#include "test.h"

void test_tracker(){
	int cond;
	char* m;
	
	struct data d1 = {{"127.0.0.1", 3212, 0}, 
	1024, 16, "HSAH", "filename.ext"};	

	struct data d2 = {{"127.0.0.2", 3212, 0}, 
	4096, 16, "HSAH", "filename2.ext"};	

	store(d1);
	store(d2);

	cond = filter(bdd, "", 2048, lt);
	m = "Filter DB by operator";

	test(cond, m);
}
