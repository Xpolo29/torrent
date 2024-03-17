#include "tracker.h"
#include "logging.h"
#include "parameters.h"

//Catch ctrl+c for clean exit
void sigint_handler(int signum) {
	if(signum !=  SIGINT)return;
	logging(LOG, "Ctrl+c received, exiting\n");
	running--;
	if(running < -1){
		logging(WARNING, "Double ctrl+c received, forcing exit\n");
		exit(6);
	}
}

//TODO Need to move this to right .c and .h
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


//TODO Need to move this to right .c and .h
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

//Handle request comprehension and answers for peer <connection>
int process(int connection){
	char buff[16*1024] = {0};
	int read = recv(connection, buff, 1024*16, 0);
	if(read < 0){
		logging(ERROR, "Could not read from socket %d\n", connection);
		return 3;
	}

	logging(LOG, "< %s\n", buff);

	//TODO parse then process then answer
	//mimic worload
	sleep(1);

	close(connection);
	return 0;
}

