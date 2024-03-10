#include <stdint.h>

#ifndef BDD_SIZE
#define BDD_SIZE 64
#endif

struct host {
  char ip[16];
  int16_t port;
};

struct data {
  struct host host;
  long size;
  int chunk_size;
  char hash[64];
  char filename[352];
};

static const struct data EMPTY = {"", 0, 0, 0, "", ""};

int get_size();

int store(struct data);
void load_all(struct data *);
int load_files(struct data *, char *filename);
struct data load_hash(char hash[64]);
int remove_host(struct host);
int remove_file(char *filename);
int remove_hash(char hash[64]);
