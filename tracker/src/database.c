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
	if (filesize == 0) {
		if(strlen(filename) == 0){
			load_all(list);
			return get_size();
		}
		else{
			return load_files(list, filename) - 1;
		}
	}
	if (strlen(filename) == 0) {
		struct data all[BDD_SIZE];
		int len = get_size();
		load_all(all);
		return compare(all, list, filesize, op, len);
	} else {
		struct data all[BDD_SIZE];
		int len = load_files(all, filename);
		return compare(all, list, filesize, op, len);
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
	printf("ip : %s, port : %d, size : %ld, chunk_size : %d, hash : %s, filename : %s\n", d.host.ip, d.host.port, d.size, d.chunk_size, d.hash, d.filename);
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
// remove e in bdd based on host, return true on success
int remove_host(struct host host) {
  int res = 0;
  for (int i = 0; i < BDD_SIZE; ++i) {
    if (*(uint32_t *)&host == *(uint32_t *)&bdd[i]) {
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
