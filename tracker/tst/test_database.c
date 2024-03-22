#include "test.h"
#include "../src/database.h"

void test_database(){
	int cond;
	char* m;

	char HASH[64] = "";

	cond = get_size() == 0;
	m = "DB initialisation";
	test(cond, m);
	
	struct data d1 = {
		{"127.0.0.1", 2222},
	       	128, 16, "", "filename.ext"
	};
	strcpy(d1.hash,HASH);

	cond  = store(d1) && get_size() == 1;
	m = "Storing in DB";
	test(cond, m);
	
	cond = remove_host(d1.host) && get_size() == 0;	
	m = "Removing by host";
	test(cond, m);

	cond = store(d1) && remove_file("filename.ext") && get_size() == 0;
	m = "Removing by filename";
	test(cond, m);


	cond = store(d1) && remove_hash(HASH) && get_size() == 0;
	m = "Removing by hash";
	test(cond, m);

	struct data d2 = {
		{"127.0.0.2", 2222},
	       	128, 16, "", "filename.ext"
	};

	strcpy(d2.hash,HASH);

	cond = !data_equals(d1, d2) && data_equals(d1, d1);
	m = "Struct data equality";
	test(cond, m);

	struct data DB_COPY[BDD_SIZE];
	store(d1);
	store(d1);
	load_all(DB_COPY);
	cond = data_equals(DB_COPY[0], DB_COPY[1]); 
	m = "Retrieving all DB";
	test(cond, m);

}
