#include "tracker.h"
#include <stdio.h>
#include <string.h>

int compare(struct data *in, struct data *out, long filesize, enum op_t op,
            int len) {
  int count = 0;
  for (int i = 0; i < len; i++) {
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
  if (filesize < 0) {
    return load_files(list, filename);
  }
  if (strlen(filename) == 0) {
    struct data *all;
    int len = get_size();
    load_all(all);
    return compare(all, list, filesize, op, len);
  } else {
    struct data *all;
    int len = load_files(all, filename);
    return compare(all, list, filesize, op, len);
  }
}

int main(){};
