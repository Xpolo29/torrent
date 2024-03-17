#include "parser.h"
#include "database.h"

//TODO : documentation
enum request_t char_to_req(char *request) {
  if (strcmp(request, "announce") == 0) {
    return announce;
  } else if (strcmp(request, "look") == 0) {
    return look;
  } else if (strcmp(request, "getfile") == 0) {
    return getfile;
  } else if (strcmp(request, "update") == 0) {
    return update;
  } else {
    // logging(ERROR, "Failed to convert char to enum request_t\n");
    return -1; // Return an error value
  }
}

//TODO : documentation
char *parse_request(char *buf, char *request, int peer_id) {
  char *reg_update = "^(update) seed \\[(([[:alnum:]]* ?)*)\\] leech "
                     "\\[(([[:alnum:]]+ ?)*)\\]$";
  char *reg_look = "^(look) (\\[(filename='([[:graph:]]+)')? "
                   "?(filesize([<=>])'([[:digit:]]+)')?\\])$";
  char *reg_get_file = "^(getfile) ([[:alnum:]]+)$";
  char *reg_announce =
      "^(announce) listen ([[:digit:]]{4}) seed \\[(([[:graph:]]+ [[:digit:]]+ "
      "[[:digit:]]+ [[:alnum:]]+ ?)*)\\] leech \\[(([[:alnum:]]+ ?)*)\\]$";

  char *all_reg[4] = {reg_update, reg_look, reg_get_file, reg_announce};

  regex_t regex;
  regmatch_t matches[MATCH_SIZE];
  int index[MATCH_SIZE][2] = {};
  int result;
  int nb_matches = 0;
  // logging(DEBUG, "Compiling regex\n");
  for (int i = 0; i < 5; i++) {
    // Si aucun regex ne reconnait la requête : Erreur de syntaxe
    if (i == 4)
      return "Error";
    result = regcomp(&regex, all_reg[i], REG_EXTENDED);
    if (result) {
      // logging(ERROR, ("Couldn't Compile Regex\n"));
      printf("Can't Compile\n");
      regfree(&regex);
      exit(1);
    }
    printf("Just compiled regex\n");
    result = regexec(&regex, request, MATCH_SIZE, matches, 0);
    regfree(&regex);
    if (!result) {
      for (int j = 1; j < MATCH_SIZE; j++) {
        int start = matches[j].rm_so;
        int end = matches[j].rm_eo;
        if (start == -1 && end == -1) {
          break;
        } else {
          index[j - 1][0] = start;
          index[j - 1][1] = end;
          nb_matches++;
          i = 5;
        }
      }
    } else if (result != REG_NOMATCH) {
      // logging(ERROR, "ERROR while matching regex\n");
      printf("Could not match %d\n", i);
      exit(1);
    }
  }
  for (int i = 0; i < nb_matches; i++) {
    int start = index[i][0];
    int size = index[i][1] - index[i][0];
    printf("start : %d, size %d\n", start, size);
    char message[size];
    memcpy(message, request + start, size);
    printf("Group %d : %s\n", i, message);
  }

  int start = index[0][0];
  int size = index[0][1] - index[0][0];
  printf("start : %d, size %d\n", start, size);

  char req[size];
  memcpy(req, request + start, size);
  req[size] = 0;
  printf("req : %s\n", req);
  enum request_t reqt = char_to_req(req);
  printf("reqt : %d\n", reqt);

  switch (reqt) {
  case getfile: {
    start = index[1][0];
    size = index[1][1] - index[1][0];
    char hash[size];
    memcpy(hash, request + start, size);
    hash[size] = 0;
    // return process_getfile(buf,hash);
    printf("hash : %s\n", hash);
    exit(0);
    break;
  }
  case look: {
    start = index[3][0];
    size = index[3][1] - index[3][0];
    char filename[size];
    memcpy(filename, request + start, size);
    filename[size] = 0;

    start = index[5][0];
    char op = request[start];

    start = index[6][0];
    size = index[6][1] - index[6][0];
    char filesize_c[size];
    memcpy(filesize_c, request + start, size);
    filesize_c[size] = 0;
    int filesize = atoi(filesize_c);

    // return process_look(buf,filename, op, filesize);
    printf("look filename: %s filesize%c%d\n", filename, op, filesize);
    exit(0);
    break;
  }
  case update: {
    printf("update\n");
    start = index[1][0];
    size = index[1][1] - index[1][0];
    char seed[size];
    memcpy(seed, request + start, size);
    seed[size] = 0;
    char *token = strtok(seed, " ");
    char seeds[MAX_SEED][HASH_SIZE];
    int seed_size = 0;
    while (token != NULL) {
      memcpy(seeds[seed_size], token, HASH_SIZE);
      seeds[seed_size][HASH_SIZE - 1] = 0;
      token = strtok(NULL, " ");
      seed_size++;
    }

    start = index[3][0];
    size = index[3][1] - index[3][0];
    char leech[size];
    memcpy(leech, request + start, size);
    leech[size] = 0;
    token = strtok(leech, " ");
    char leeches[MAX_SEED][HASH_SIZE + 1];
    int leech_size = 0;
    while (token != NULL) {
      memcpy(leeches[leech_size], token, HASH_SIZE + 1);
      leeches[leech_size][HASH_SIZE] = 0;
      token = strtok(NULL, " ");
      leech_size++;
    }
    // return process_update(buf,seeds, seed_size, leeches, leech_size);
    printf("Seeds : ");
    for (int i = 0; i < seed_size; i++) {
      printf("%s ", seeds[i]);
    }
    printf("\n");
    printf("Leeches : ");
    for (int i = 0; i < leech_size; i++) {
      printf("%s ", leeches[i]);
    }
    printf("\n");
    exit(0);
    break;
  }
  case announce: {
    start = index[2][0];
    size = index[2][1] - index[2][0];
    char seed[size];
    memcpy(seed, request + start, size);
    seed[size] = 0;
    char *token = strtok(seed, " ");
    struct data seeds[MAX_SEED];
    int seed_size = 0;
    int modulo = 0;
    while (token != NULL) {
      switch (modulo) {
      case 0: {
        strncpy(seeds[seed_size].filename, token, HASH_SIZE);
        break;
      }
      case 1: {
        char size_c[HASH_SIZE];
        strncpy(size_c, token, HASH_SIZE);
        size_c[HASH_SIZE - 1] = 0;
        seeds[seed_size].size = atol(size_c);
        break;
      }
      case 2: {
        char size_c[HASH_SIZE];
        strncpy(size_c, token, HASH_SIZE);
        size_c[HASH_SIZE - 1] = 0;
        seeds[seed_size].chunk_size = atoi(size_c);
        break;
      }
      case 3: {
        strncpy(seeds[seed_size].hash, token, HASH_SIZE);
        break;
      }
      }
      modulo = (modulo + 1) % 4;
      token = strtok(NULL, " ");
      if (modulo == 0)
        seed_size++;
    }

    start = index[4][0];
    size = index[4][1] - index[3][0];
    char leech[size];
    memcpy(leech, request + start, size);
    leech[size] = 0;
    token = strtok(leech, " ");
    char leeches[MAX_SEED][HASH_SIZE + 1];
    int leech_size = 0;
    while (token != NULL) {
      memcpy(leeches[leech_size], token, HASH_SIZE + 1);
      leeches[leech_size][HASH_SIZE] = 0;
      token = strtok(NULL, " ");
      leech_size++;
    }
    // return process_announce(buf,seeds, seed_size, leeches, leech_size);
    printf("Seeds : ");
    for (int i = 0; i < seed_size; i++) {
      printf("filename : %s size : %ld chunk_size : %d hash : %s\n ",
             seeds[i].filename, seeds[i].size, seeds[i].chunk_size,
             seeds[i].hash);
    }
    printf("\n");
    printf("Leeches : ");
    for (int i = 0; i < leech_size; i++) {
      printf("%s ", leeches[i]);
    }
    printf("\n");
    exit(0);
    break;
  }
  }
}

//TODO : documentation
int process_getfile(char *buf, char *hash) {
  struct data d = load_hash(hash);
  if (d.size == 0) {
    buf = "Wrong hash";
    return -1;
  }
  buf = "peers ";
  strncat(buf, hash, HASH_SIZE);
  strncat(buf, " [", 2);
  char host[22];
  // TODO : Pour plusieurs peers, changer load_hash
  sprintf(host, "%s:%d", d.host.ip, d.host.port);
  strncat(buf, "]", 1);
  return 1;
}

//TODO : No other main than main.c in src/, to test use /test
/*
int main() {
  char buf[1024];
  // printf(buf, "result : %s \n", parse_request("getfile 1234", 1));
  // parse_request(buf, "look [filename='file_a.dat' filesize>'1048576']", 1);
  // parse_request(buf, "update seed [arbdfg azeeaz azeaea] leech [aedefe
  // dfgefv]", 1);
  parse_request(buf,
                "announce listen 4444 seed [filename1.dat 12 12 azerds "
                "filename2.dat 13 13 "
                "azerty] leech [aqwzsx edcrfv]",
                1);
  printf("%s\n", buf);
  return 0;
}
*/
