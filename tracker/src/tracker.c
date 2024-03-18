#include "tracker.h"
#include "logging.h"
#include "parameters.h"

// Catch ctrl+c for clean exit
void sigint_handler(int signum) {
  if (signum != SIGINT)
    return;
  logging(LOG, "Ctrl+c received, exiting\n");
  running--;
  if (running < -1) {
    logging(WARNING, "Double ctrl+c received, forcing exit\n");
    exit(6);
  }
}

// Handle request comprehension and answers for peer <connection>
int process(int connection) {
  char buff[16 * 1024] = {0};
  int read = recv(connection, buff, 1024 * 16, 0);
  if (read < 0) {
    logging(ERROR, "Could not read from socket %d\n", connection);
    return 3;
  }

  logging(LOG, "< %s\n", buff);

  // TODO parse then process the answer

  // mimic worload
  sleep(1);

  close(connection);
  return 0;
}
