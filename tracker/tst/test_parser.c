#include "../src/parser.h"
#include "test.h"
#include <string.h>

void test_parser() {
  int cond;
  char *m;
  char buf[1024];
  struct host me = {"moi.ip", 2332};
  struct host me2 = {"moi2.ip", 2333};
  struct data d1 = {me, 100, 2, "hash", "file.file"};
  struct data d2 = {me2, 100, 2, "hash", "file2.file"};
  store(d1);
  store(d2);
  parse_request(buf, "getfile hash", me);
  cond = !strcmp(buf, "peers hash [moi.ip:2332 moi2.ip:2333]");
  m = "Getfile request";
  test(cond, m);
  strcpy(buf, "");
  // printf(buf, "result : %s \n", buf);
  parse_request(buf, "look [filename='file2.file']", me);
  // printf("result : %s \n", buf);
  cond = !strcmp(buf, "list [file2.file 100 2 hash]");
  m = "Look request with only filename";
  test(cond, m);
  strcpy(buf, "");

  parse_request(buf, "look [filesize='100']", me);
  // printf("result : %s \n", buf);
  cond = !strcmp(buf, "list [file.file 100 2 hash file2.file 100 2 hash]");
  m = "Look request with only filesize";
  test(cond, m);
  strcpy(buf, "");

  parse_request(buf, "look [filename='file2.file' filesize='100']", me);
  // printf("result : %s \n", buf);
  cond = !strcmp(buf, "list [file2.file 100 2 hash]");
  m = "Look request with both filename and filesize";
  test(cond, m);
  strcpy(buf, "");
  // parse_request(buf, "update seed [arbdfg azeeaz azeaea] leech [aedefe
  // dfgefv]", 1);
  // parse_request(buf,
  //               "announce listen 4444 seed [filename1.dat 12 12 azerds "
  //               "filename2.dat 13 13 "
  //               "azerty] leech [aqwzsx edcrfv]",
  //               1);
  // printf("%s\n", buf);
}
