#ifndef DATABASE
#define DATABASE

#include <stdint.h>
#include <string.h>
#include <time.h>
#include <arpa/inet.h>
#include "parameters.h"

#define BDD_SIZE 64



// possible operator for file filtering
enum op_t { nu = -1, eq, gt, lt };

struct host {
  char ip[16];
  int16_t port;
  long last_update;
};

struct data {
  struct host host;
  long size;
  int chunk_size;
  char hash[64];
  char filename[352];
};

// bdd but defined in database.c
extern struct data bdd[BDD_SIZE];

// in case you need an empy data field
static const struct data EMPTY = {{"", 0, 0}, 0, 0, "", ""};

// return size of bdd
int get_size();

// filter list of data based on <filename> or (<filesize> op <long>)
int filter(struct data *, char *, long, enum op_t);

// store <struct data> in bdd
int store(struct data);

// copy bdd into <struct data*>, return len
void load_all(struct data *);

// copy bdd elements matching filename into <struct data*>, return len
int load_files(struct data *, char *filename);

// copy bdd elements matching hash into <struct data*>, return len
int load_hash(struct data *d, char hash[64]);

// copy bdd elements matching host into <struct data*>, return len
int load_host(struct data *d, struct host h);

// copy bdd elements matching host into <struct data*>, return len
int load_ip(struct host *d, char ip[INET_ADDRSTRLEN]);

// remove all elements of bdd matching <host>, 0 on success
int remove_host(struct host);

// remove all elements of bdd matching <filename>, 0 on success
int remove_file(char *filename);

// remove all elements of bdd matching <hash>, return 0 on success
int remove_hash(char hash[64]);

// check for struct host equality, returns 1 if equals
int host_equals(struct host, struct host);

// check for struct data equality, returns 1 if equals
int data_equals(struct data, struct data);

//print db
void print_db();

//print data d
void print_data(struct data d);

//Check if <data> is already in db, reutrn true if is in
int db_exists(struct data);

//remove doublon from arr of len <len>, returns new size
int remove_doublon_hash(struct data* arr, int len);

//remove entries of database if peers haven't been heard since time_to_live 
void remove_old_entries();

//update ttl in host arr
void update_host(struct host*);

#endif
