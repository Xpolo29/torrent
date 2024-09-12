#include "parser.h"

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
    logging(WARNING,
            "char_to_req : Failed to convert char to enum request_t\n");
    return -1; // Return an error value
  }
}

enum op_t char_to_op(char *request) {
  if (request[0] == '=') {
    return eq;
  } else if (request[0] == '>') {
    return gt;
  } else if (request[0] == '<') {
    return lt;
  } else if (request[0] == 0) {
    return nu;
  } else {
    logging(WARNING,
            "char_to_op : Failed to convert char to enum request_t with "
            "request : %s\n",
            request);
    return -1; // Return an error value
  }
}

void process_getfile(char *buf, char *hash, struct host h) {
	struct data d[BDD_SIZE];
	load_hash(d, hash);
	strcat(buf, "peers ");
	strcat(buf, hash);
	strcat(buf, " [");
	char host[23];
	int is_local_trafic = is_local_ip(h.ip);
	for (int i = 0; d[i].size != 0; i++) {
		if (i > 0)strcat(buf, " ");

		char relative_ip[INET_ADDRSTRLEN];
		//memcpy(relative_ip, d[i].host.ip, strlen(d[i].host.ip) + 1);
		strcpy(relative_ip, d[i].host.ip);

		if(is_local_ip(relative_ip) && !is_local_trafic){
			//memcpy(relative_ip, public_ip, strlen(public_ip) + 1);
			strcpy(relative_ip, public_ip);
			logging(WARNING, "Getfile request is from local to public network, converting ip to public\n");
		}


		snprintf(host, 23, "%.15s:%hu", relative_ip, d[i].host.port);
		strcat(buf, host);
	}
	strcat(buf, "]\n");
}

void process_look(char *buf, char *filename, enum op_t op, long filesize) {
  struct data d[BDD_SIZE];
  // printf("look : %s, %d, %ld\n", filename, op, filesize);
  int len = filter(d, filename, filesize, op);
  // printf("len : %d\n", len);

  strcat(buf, "list [");
  char info[1024];
  for (int i = 0; i < len; i++) {
    // printf("f:%s s:%ld cs:%d h:%s\n", d[i].filename, d[i].size,
    // d[i].chunk_size, d[i].hash);
    if (d[i].size == 0)
      continue; // pass empty
    if (i > 0)
      strcat(buf, " ");

    sprintf(info, "%s %ld %d %s", d[i].filename, d[i].size, d[i].chunk_size,
            d[i].hash);
    strcat(buf, info);
  }
  strcat(buf, "]\n");
}

void process_update(char *buf, struct data *seeds, int seed_size, struct data *leeches, int leech_size, struct host h) {
	//printf("seed_size : %d, leech_side : %d\n", seed_size, leech_size);
	struct data dbb_host[BDD_SIZE];
	struct data new_host[BDD_SIZE];
	int new_len = 0;
	int len = get_size();
	load_all(dbb_host);

	for (int i = 0; i < len; i++) {
		for (int j = 0; j < seed_size; j++) {
			if ((strcmp(dbb_host[i].hash, seeds[j].hash) == 0)) {
				new_host[new_len++] = dbb_host[i];
			}
		}
		for (int j = 0; j < leech_size; j++) {
			if (strcmp(dbb_host[i].hash, leeches[j].hash) == 0) {
				new_host[new_len++] = dbb_host[i];
			}
		}
	}

	remove_host(h);
	for (int i = 0; i < new_len; i++) {
		new_host[i].host = h;
		if (!db_exists(new_host[i]))
			store(new_host[i]);
	}
	strcpy(buf, "ok\n");
}

void process_announce(char *buf, struct data *seeds, int seed_size,
                      struct data *leeches, int leech_size) {
  // to get rid of warning
  // (void)leeches;
  // (void)leech_size;
  // printf("seed_size : %d", seed_size);
  struct data all[BDD_SIZE];
  load_all(all);
  int size = get_size();
  for (int i = 0; i < seed_size; i++) {
    if (!db_exists(seeds[i]))
      store(seeds[i]);
  }

  for (int i = 0; i < leech_size; i++) {
    for (int j = 0; j < size; j++) {
      if (strcmp(all[j].hash, leeches[i].hash) == 0 &&
          !host_equals(all[j].host, leeches[i].host)) {
        leeches[i].size = all[j].size;
        strcpy(leeches[i].filename, all[j].filename);
        leeches[i].chunk_size = all[j].chunk_size;
        store(leeches[i]);
        break;
      }
    }
  }
  strcpy(buf, "ok\n");
}

int handle_regex(int index[MATCH_SIZE][2], char *request) {
  char *reg_update = "^(update) (seed \\[(([[:alnum:]]* ?)*)\\])? ?(leech "
                     "\\[(([[:alnum:]]+ ?)*)\\])?((\r)?(\n)?)?$";
  char *reg_look =
      "^(look) (\\[(filename=[\"']([[:graph:]]+)[\"'])? "
      "?(filesize([<=>])[\"']([[:digit:]]+)[\"'])?\\])((\r)?(\n)?)?$";
  char *reg_get_file = "^(getfile) ([[:alnum:]]+)((\r)?(\n)?)?$";
  char *reg_announce =
      "^(announce) listen ([[:digit:]]+) ?(seed \\[(([[:graph:]]+ [[:digit:]]+ "
      "[[:digit:]]+ [[:alnum:]]+ ?)*)\\])? ?(leech \\[(([[:alnum:]]+ "
      "?)*)\\])?((\r)?(\n)?)?$";

  char *all_reg[4] = {reg_update, reg_look, reg_get_file, reg_announce};

  regex_t regex;
  regmatch_t matches[MATCH_SIZE];
  int result;
  int nb_matches = 0;
  logging(DEBUG, "Parser : Compiling regex\n");
  for (int i = 0; i < 5; i++) {
    // Si aucun regex ne reconnait la requête : Erreur de syntaxe
    if (i == 4) {
      logging(WARNING, "Parser : No pattern matching\n");
      return 1;
    }
    result = regcomp(&regex, all_reg[i], REG_EXTENDED);
    if (result) {
      logging(ERROR, ("Parser : Error while compiling Regex\n"));
      // printf("Can't Compile\n");
      regfree(&regex);
      exit(1);
    }
    // printf("Just compiled regex\n");
    logging(DEBUG, "Parser : Regex compiled successfully\n");
    result = regexec(&regex, request, MATCH_SIZE, matches, 0);
    regfree(&regex);
    if (!result) {
      for (int j = 1; j < MATCH_SIZE; j++) {
        int start = matches[j].rm_so;
        int end = matches[j].rm_eo;
        if (start == -1 && end == -10) {
          break;
        } else {
          index[j - 1][0] = start;
          index[j - 1][1] = end;
          nb_matches++;
          i = 5;
        }
      }
    } else if (result != REG_NOMATCH) {
      logging(ERROR, "Parser : Regex matching\n");
      // printf("Could not match %d\n", i);
      exit(1);
    }
  }
  // DEBUG purpose
/*  
  for (int i = 0; i < nb_matches; i++) {
    int start = index[i][0];
    int size = index[i][1] - index[i][0];
    printf("start : %d, size %d\n", start, size);
    char message[size];
    memcpy(message, request + start, size);
    printf("Group %d : %s\n", i, message);
  }
  */
  return 0;
}

void parse_getfile(char *buf, int index[MATCH_SIZE][2], char *request, struct host h) {
  logging(DEBUG, "Parser : Processing getfile request\n");
  int start = index[1][0];
  int size = index[1][1] - index[1][0];
  char hash[size];
  memcpy(hash, request + start, size);
  hash[size] = 0;
  // printf("hash : %s\n", hash);
  process_getfile(buf, hash, h);
}

void parse_look(char *buf, int index[MATCH_SIZE][2], char *request) {
  logging(DEBUG, "Parser : Look request\n");

  int filename_match;
  int filesize_match;
  // to get rid of warning
  (void)filename_match;
  (void)filesize_match;

  int start = index[3][0];
  int size = index[3][1] - index[3][0];
  char filename[352];
  strncpy(filename, request + start, size);
  filename[size] = 0;

  start = index[5][0];
  char op[1];
  if (index[5][1] == 0) {
    op[0] = 0;
  } else
    op[0] = request[start];

  start = index[6][0];
  size = index[6][1] - index[6][0];

  char filesize_c[size + 1];

  strncpy(filesize_c, request + start, size);
  filesize_c[size] = 0;
  int filesize = atoi(filesize_c);
  // printf("filesize : %d\n", filesize);

  process_look(buf, filename, char_to_op(op), filesize);
  // printf("look filename: %s filesize%s%d\n", filename, op, filesize);
}

void parse_update(char *buf, int index[MATCH_SIZE][2], char *request,
                  struct host h) {
  logging(DEBUG, "Parser : Update request\n");
  int start = index[2][0];
  int size = index[2][1] - index[2][0];
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

  start = index[5][0];
  size = index[5][1] - index[5][0];
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
  struct data seeds_d[seed_size];
  struct data leech_d[leech_size];

  for (int i = 0; i < seed_size; i++) {
    strcpy(seeds_d[i].hash, seeds[i]);
  }
  for (int i = 0; i < leech_size; i++) {
    strcpy(leech_d[i].hash, leeches[i]);
  }
  process_update(buf, seeds_d, seed_size, leech_d, leech_size, h);
  // printf("Seeds : ");
  // for (int i = 0; i < seed_size; i++) {
  //   printf("%s ", seeds[i]);
  // }
  // printf("\n");
  // printf("Leeches : ");
  // for (int i = 0; i < leech_size; i++) {
  //   printf("%s ", leeches[i]);
  // }
  // printf("\n");
  // exit(0);
}

void parse_announce(char *buf, int index[MATCH_SIZE][2], char *request,
                    struct host h) {
  logging(DEBUG, "Parser : announce request\n");

  int start = index[1][0];
  int size = index[1][1] - index[1][0];

  char portchar[6];
  memcpy(portchar, request + start, size);
  int port = atoi(portchar);

  start = index[3][0];
  size = index[3][1] - index[3][0];
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
    if (modulo == 0) {
      seeds[seed_size].host = h;
      seeds[seed_size].host.port = port;
      seed_size++;
    }
  }

  start = index[6][0];
  size = index[6][1] - index[6][0];
  char leech[size];
  int leech_size = 0;
  char leeches[MAX_SEED][HASH_SIZE + 1];
  if (size != 0) {
    memcpy(leech, request + start, size);
    leech[size] = 0;
    token = strtok(leech, " ");
    while (token != NULL) {
      memcpy(leeches[leech_size], token, HASH_SIZE + 1);
      leeches[leech_size][HASH_SIZE] = 0;
      token = strtok(NULL, " ");
      leech_size++;
    }
  }
  struct data leech_d[MAX_SEED];
  for (int i = 0; i < leech_size; i++) {
    strcpy(leech_d[i].hash, leeches[i]);
    leech_d[i].host = h;
    leech_d[i].host.port = port;
  }
  process_announce(buf, seeds, seed_size, leech_d, leech_size);
  // printf("Seeds : ");
  // for (int i = 0; i < seed_size; i++) {
  //   printf("filename : %s size : %ld chunk_size : %d hash : %s\n ",
  //          seeds[i].filename, seeds[i].size, seeds[i].chunk_size,
  //          seeds[i].hash);
  // }
  // printf("\n");
  // printf("Leeches : ");
  // for (int i = 0; i < leech_size; i++) {
  //   printf("%s ", leeches[i]);
  // }
  // printf("\n");
  // exit(0);
}

int parse_request(char *buf, char *request, struct host h) {
  int index[MATCH_SIZE][2] = {};
  int res = handle_regex(index, request);
  if (res == 1)
    return 1;

  int start = index[0][0];
  int size = index[0][1] - index[0][0];
  // printf("start : %d, size %d\n", start, size);

  char req[size];
  memcpy(req, request + start, size);
  req[size] = 0;
  // printf("req : %s\n", req);
  enum request_t reqt = char_to_req(req);
  // printf("reqt : %d\n", reqt);

  switch (reqt) {
  case getfile: {
    parse_getfile(buf, index, request, h);
    break;
  }
  case look: {
    parse_look(buf, index, request);
    break;
  }
  case update: {
    parse_update(buf, index, request, h);
    break;
  }
  case announce: {
    parse_announce(buf, index, request, h);
    break;
  }
    return 0;
  }
  return 0;
}

// TODO : No other main than main.c in src/, to test use /test
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
