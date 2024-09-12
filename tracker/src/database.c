#include "database.h"
#include <stdio.h>

// bdd
struct data bdd[BDD_SIZE];

int compare(struct data *in, struct data *out, long filesize, enum op_t op, int len) {
	int count = 0;
	for (int i = 0; i < len; i++) {
	if(in[i].size == 0)continue; //skip empty
	switch (op) {
		case eq:
			if (in[i].size == filesize)
			out[count++] = in[i];
			break;
		case gt:
			if (in[i].size > filesize)
			out[count++] = in[i];
			break;
		case lt:
			if (in[i].size < filesize)
			out[count++] = in[i];
			break;
		default:
			break;
	}
	}
	return count;
}

int filter(struct data *list, char *filename, long filesize, enum op_t op) {

	//printf("Filename %s and size %ld\n", filename, strlen(filename));
	if (filesize == 0) {
		if(strlen(filename) == 0){
			load_all(list);
			return get_size();
		}
		else{
			int len = load_files(list, filename) - 1;
			return remove_doublon_hash(list, len);
		}
	}

	if (strlen(filename) == 0) {
		struct data all[BDD_SIZE];
		int len = get_size();
		load_all(all);
		len = compare(all, list, filesize, op, len);
		/*
		printf("LEN : %d\n", len);
		for(int i = 0; i < BDD_SIZE; ++i){
			if (list[i].size != 0)
			print_data(list[i]);
		}
		*/
		return remove_doublon_hash(list, len);
	} else {
		struct data all[BDD_SIZE];
		int len = load_files(all, filename);
		compare(all, list, filesize, op, len);
		return remove_doublon_hash(list, len);
	}
}

// return how many element are stored in bdd
int get_size() {
  int size = 0;
  for (int i = 0; i < BDD_SIZE; ++i)
    if (bdd[i].size != 0)
      ++size;

  return size;
}

// store e in bdd, return true on success
int store(struct data e) {
	for (int i = 0; i < BDD_SIZE; ++i) {
		if (bdd[i].size == 0) {
			bdd[i] = e;
			return 1;
		}
	}
	return 0;
}

void print_data(struct data d){
	printf("ip : %s, port : %u, last_seen : %ld, size : %ld, chunk_size : %d, hash : %s, filename : %s\n", d.host.ip, d.host.port, d.host.last_update, d.size, d.chunk_size, d.hash, d.filename);
}

void print_db(){
	for(int i = 0; i < BDD_SIZE; ++i){
	    if (bdd[i].size != 0)
		    print_data(bdd[i]);
	}
}

int host_equals(struct host h1, struct host h2){
	int ip_eq = !strcmp(h1.ip, h2.ip);
	int port_eq = h1.port = h2.port;
	return ip_eq && port_eq;
}

int data_equals(struct data d1, struct data d2) {

	int host_eq = host_equals(d1.host, d2.host);
	int size_eq = d1.size == d2.size;
	int chunk_size_eq = d1.chunk_size == d2.chunk_size;
	int hash_eq = !strcmp(d1.hash, d2.hash);
	int filename_eq = !strcmp(d1.filename, d2.filename);

	return host_eq && size_eq && chunk_size_eq && hash_eq && filename_eq;
}

int db_exists(struct data h){
	struct data clone[BDD_SIZE];
	load_all(clone);
	for(int i = 0; i < BDD_SIZE; ++i){
		if(data_equals(clone[i], h))
				return 1;
	}
	return 0;
}
// load bdd into arr
void load_all(struct data *arr) {
  memcpy(arr, bdd, sizeof(struct data) * BDD_SIZE);
}

// return len of arr of element matching filename
int load_files(struct data *arr, char *filename) {
	int index = 0;
	for (int i = 0; i < BDD_SIZE; ++i) {
		if (strcmp(bdd[i].filename, filename) == 0) {
			arr[index] = bdd[i];
			++index;
		}
	}
	return index + 1;
}

// return element matching hash
int load_hash(struct data *d, char hash[64]) {
  int c = 0;
  for (int i = 0; i < BDD_SIZE; ++i) {
    // printf("bdd : %s, main %s = %d\n", bdd[i].hash, hash,
    // strcmp(bdd[i].hash, hash));
    if (strcmp(bdd[i].hash, hash) == 0) {
      d[c++] = bdd[i];
    }
  }
  d[c] = EMPTY;
  return c;
}

int load_host(struct data *d, struct host h){
	int c = 0;
	for (int i = 0; i < BDD_SIZE; ++i) {
		struct host temp = bdd[i].host;
		if(!strcmp(temp.ip, h.ip) && temp.port == h.port){
			d[c++] = bdd[i];
		}
	}
	d[c] = EMPTY;
	return c;
}

int load_ip(struct host *d, char ip[INET_ADDRSTRLEN]){
	int c = 0;
	for (int i = 0; i < BDD_SIZE; ++i) {
		struct host temp = bdd[i].host;
		if(!strcmp(temp.ip, ip)){
			d[c++] = bdd[i].host;
		}
	}
	d[c] = EMPTY.host;
	return c;
}

int remove_doublon_hash(struct data* arr, int len){
	struct data copy[BDD_SIZE] ;
	memcpy(copy, arr, len* sizeof(struct data));
	memset(arr, 0, BDD_SIZE * sizeof(struct data));

	char hashes[BDD_SIZE][64];
	int hash_len = 0;
	int skip = 0;

	for(int i = 0; i < len; ++i){
		skip = 0;
		char cur_hash[64];
		strcpy(cur_hash, copy[i].hash);

		for(int j = 0; j < hash_len; j++){
			if(!strcmp(hashes[j], cur_hash)){
				skip = 1;
				break;
			}
		}

		if(skip)continue;

		strcpy(hashes[hash_len], cur_hash);
		arr[hash_len] = copy[i];
		hash_len++;
	}

	return hash_len;
}

// remove e in bdd based on host, return true on success
int remove_host(struct host host) {
	int res = 0;
	for (int i = 0; i < BDD_SIZE; ++i) {
		if(host_equals(host, bdd[i].host)){
			bdd[i] = EMPTY;
			res = 1;
		}
	}
	return res;
}

// remove e in bdd based on filename, return true on success
int remove_file(char *filename) {
  int res = 0;
  for (int i = 0; i < BDD_SIZE; ++i) {
    if (strcmp(filename, bdd[i].filename) == 0) {
      bdd[i] = EMPTY;
      res = 1;
    }
  }
  return res;
}

// remove e in bdd based on hash, return true on sucess
int remove_hash(char hash[64]) {
  for (int i = 0; i < BDD_SIZE; ++i) {
    if (strcmp(hash, bdd[i].hash) == 0) {
      bdd[i] = EMPTY;
      return 1;
    }
  }
  return 0;
}

void update_host(struct host* h){
	long now = time(NULL);
	h->last_update = now;
}

void remove_old_entries(){
	long now = time(NULL);
	for(int i = 0; i < BDD_SIZE; ++i){
		struct host curh = bdd[i].host;
		if(curh.last_update == 0)continue;

		long diff = now - curh.last_update;

		if( diff > time_to_live){
			remove_host(curh);
		}
	}	
}
