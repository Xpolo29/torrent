#include <stdio.h>

#define R "\x1b[31m"
#define G "\x1b[32m"
#define W "\x1b[0m"

#define test(cond, msg) ((cond) ? ok(msg) : no(msg))

extern int cur_test;
extern const int max_test;

void ok(char *);
void no(char *);

void test_database();
void test_parser();
void test_logging();
void test_network();
void test_threads();
void test_tracker();
