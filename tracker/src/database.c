#include "database.h"

//bdd
struct data bdd[BDD_SIZE];

//return how many element are stored in bdd
int get_size(){
	int size = 0;
	for(int i = 0; i < BDD_SIZE; ++i)
		if(bdd[i].size != 0)++size;
	
	return size;
}

//store e in bdd, return true on success
int store(struct data e){
	for(int i = 0; i < BDD_SIZE; ++i){
		if(bdd[i].size == 0){
			bdd[i] = e;
			return 1;
		}
	}
	return 0;
}

//load bdd into arr
void load_all(struct data* arr){
	memcpy(arr, bdd, sizeof(struct data) * BDD_SIZE);
}

//return len of arr of element matching filename
int load_files(struct data* arr, char* filename){
	int index = 0;
	for(int i = 0; i < BDD_SIZE; ++i){
		if(strcmp(bdd[i].filename, filename) == 0){
			arr[index] = bdd[i];
			++index;
		}
	}
	return index + 1;	
}

//return element matching hash
struct data load_hash(char hash[64]){
	for(int i = 0; i < BDD_SIZE; ++i){
		if(strcmp(bdd[i].hash, hash) == 0){
			return bdd[i];
		}
	}
	return EMPTY;
}

//remove e in bdd based on host, return true on success
int remove_host(struct host host){
	int res = 0;
	for(int i = 0; i < BDD_SIZE; ++i){
		if(*(uint32_t*)&host == *(uint32_t*)&bdd[i]){
			bdd[i] = EMPTY;
			res = 1;
		}
	}
	return res;
}

//remove e in bdd based on filename, return true on success
int remove_file(char* filename){
	int res = 0;
	for(int i = 0; i < BDD_SIZE; ++i){
		if(strcmp(filename, bdd[i].filename) == 0){
			bdd[i] = EMPTY;
			res = 1;
		}
	}
	return res;
}

//remove e in bdd based on hash, return true on sucess
int remove_hash(char hash[64]){
	for(int i = 0; i < BDD_SIZE; ++i){
		if(strcmp(hash, bdd[i].hash) == 0){
			bdd[i] = EMPTY;
			return 1;
		}
	}
	return 0;
}
