#include "../src/parser.h"
#include "test.h"
#include <string.h>

void test_parser() {
  int cond;
  char *m;
  char buf[1024];
  struct host me = {"moi.ip", 2332, 0};
  struct host me2 = {"moi2.ip", 2333, 0};
  struct data d1 = {me, 100, 2, "hash", "file.file"};
  struct data d2 = {me2, 100, 2, "hash", "file.file"};
  struct data d3 = {me2, 200, 2, "hash2", "file3.file"};
  store(d1);
  store(d2);
  store(d3);

  parse_request(buf, "getfile hash", me);
  cond = !strcmp(buf, "peers hash [moi.ip:2332 moi2.ip:2333]\n");
  m = "Getfile request";
  test(cond, m);
  strcpy(buf, "");
  // printf(buf, "result : %s \n", buf);

  parse_request(buf, "look [filename='file.file']", me);
  // printf("result : %s \n", buf);
  cond = !strcmp(buf, "list [file.file 100 2 hash]\n");
  m = "Look request with only filename";
  test(cond, m);
  strcpy(buf, "");

  //printf("request : look [filesize='100']\n");
  parse_request(buf, "look [filesize=\"100\"]", me);
  //printf("result : %s \n", buf);
  //printf("Wanted : list [file.file 100 2 hash]\n");
  cond = !strcmp(buf, "list [file.file 100 2 hash]\n");
  m = "Look request with only filesize";
  test(cond, m);
  strcpy(buf, "");

  parse_request(buf, "look [filename='file.file' filesize='100']\n", me);
  // printf("result : %s \n", buf);
  cond = !strcmp(buf, "list [file.file 100 2 hash]\n");
  m = "Look request with both filename and filesize";
  test(cond, m);
  strcpy(buf, "");

  // print_db();
  int len1 = get_size();
  parse_request(buf, "update seed [hash]\n", me2);
  parse_request(buf, "update seed [hash]\n", me2);
  // print_db();
  // printf("result : %s \n", buf);
  int len2 = get_size();
  cond = len2 < len1;
  m = "Update request double seed only";
  test(cond, m);
  strcpy(buf, "");

  // print_db();
  len1 = get_size();
  parse_request(buf, "update seed [] leech []\n", me2);
  // print_db();
  // printf("result : %s \n", buf);
  len2 = get_size();
  cond = len2 < len1;
  m = "Update request seed + leech";
  test(cond, m);
  strcpy(buf, "");

  // print_db();
  len1 = get_size();
  parse_request(buf, "update leech [hash]\n", me2);
  // print_db();
  // printf("result : %s \n", buf);
  len2 = get_size();
  cond = len2 > len1;
  m = "Update request leech only";
  test(cond, m);
  strcpy(buf, "");
  // print_db();
  // remove_host(me2);
  // len1 = get_size();
  // parse_request(buf, "update leech [hash]\n", me2);
  // print_db();
  // // printf("result : %s \n", buf);
  // len2 = get_size();
  // cond = len2 > len1;
  // m = "Update request";
  // printf("len1 : %d, len2 : %d", len1, len2);
  // test(cond, m);
  // strcpy(buf, "");

  len1 = get_size();
  parse_request(buf, "announce listen 2333\n", me2);
  len2 = get_size();
  m = "Announce Request no seed no leech";
  cond = len2 == len1;
  // printf("len1 : %d, len2 : %d", len1, len2);
  test(cond, m);
  // print_db();
  strcpy(buf, "");

  len1 = get_size();
  parse_request(buf, "announce listen 2333 seed [file3.file 10 1 hash3]\n",
                me2);
  len2 = get_size();
  m = "Announce Request seed only";
  cond = len2 > len1;
  // printf("len1 : %d, len2 : %d", len1, len2);
  test(cond, m);
  // print_db();
  strcpy(buf, "");

  remove_host(me2);
  len1 = get_size();
  parse_request(
      buf, "announce listen 2333 seed [file3.file 10 1 hash3] leech [hash]\n",
      me2);
  len2 = get_size();
  m = "Announce Request seed and leech";
  cond = len2 > len1;
  // printf("len1 : %d, len2 : %d", len1, len2);
  test(cond, m);
  // print_db();
  strcpy(buf, "");

  remove_host(me2);
  len1 = get_size();
  parse_request(buf, "announce listen 2333 leech [hash]\n", me2);
  len2 = get_size();
  m = "Announce Request leech only";
  cond = len2 > len1;
  // printf("len1 : %d, len2 : %d", len1, len2);
  test(cond, m);
  // print_db();
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
