#include "test.h"

int cur_test = 1;
const int max_test = 18;

void ok(char *s) {
  printf(G "PASSED (%d/%d): %s\n" W, cur_test, max_test, s);
  cur_test++;
}

void no(char *s) {
  printf(R "FAILED (%d/%d): %s\n" W, cur_test, max_test, s);
  cur_test++;
}

int main() {

  printf("##### DATABASE tests #####\n");
  test_database();

  printf("##### LOGGING tests #####\n");
  test_logging();

  printf("##### NETWORK tests #####\n");
  test_network();

  printf("##### THREADS tests #####\n");
  test_threads();

  printf("##### TRACKER tests #####\n");
  test_tracker();

  printf("##### PARSER tests #####\n");
  test_parser();

  return 0;
};
