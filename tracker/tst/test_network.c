#include "../src/network.h"
#include "test.h"

void test_network(){
	int cond;
	char* m;

	cond = create_master_sock(3412) >= 0;
	m = "Creating listening socket";
	test(cond, m);
}
