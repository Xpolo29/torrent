#include "../src/logging.h"
#include "test.h"

void test_logging(){
	int cond;
	char* m;
	
	cond = !strcmp(log_level_to_string(WARNING), "WARNING");
	m = "Log level to string conversion";
	test(cond, m);

}
