#include <stdint.h>

struct host{
 	char ip[16];
	int16_t port;
};
	
struct data{
	struct host host;
	int size;
	int chunk_size;
	char hash[64];
	char filename[352];
};

int store(struct data);
struct data* loadAll(void);	
struct data* loadFiles(char* filename);	
struct data loadHash(char hash[64]);	
int remove(struct host); 
int remove(char* filename);
int remove(char hash[64]);

