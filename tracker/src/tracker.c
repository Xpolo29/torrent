#include "tracker.h"
#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <stdlib.h>
#include <time.h>
#include <stdarg.h>
#include <dirent.h>
#include <sys/stat.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>
#include <signal.h>
#include <fcntl.h>

//global var
char* config_path = "config.ini";
enum LOG_LEVEL log_level = ERROR;
int16_t port = -1;
int running = 1;

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

//used to parse args on cmd
int parse_args(int argc, char** argv){
	for(int i = 1; i < argc; ++i){
		for(int j = 0; j < LEN_ARGS; ++j){
			if(strcmp(ARGS[j], argv[i]) == 0){
				switch(j){
					case 0: // -v
					case 1: // --verbose
						// check if there is a value
						if(i + 1 < argc && isdigit(argv[i+1][0])){
							log_level = atoi(argv[i + 1]) % 4;
							++i;
						}else{
							if(log_level == -1){log_level = 2;}
							else{log_level =(log_level + 1) % 4;}
						}
						break;
					case 2: // -h
					case 3: // --help
						printf("This is the help message\n");
						exit(1);
						break;
					case 4: // -p
					case 5: // --port
						if(i + 1 < argc && isdigit(argv[i+1][0])){
							port = atoi(argv[i + 1]);
							logging(LOG, "Port updated to %d\n", port);
							++i;
						}else{
							logging(WARNING, "Got -p but no port is specified, using default\n");
						}
						break;
					case 6: // -c
					case 7: // --config
						if(i + 1 < argc){
							config_path = argv[i + 1];
							logging(LOG, "Config path updated to %s\n", config_path);
							++i;
						}else{
							logging(WARNING, "Got -c but no config path is specified, using default\n");
						}
						break;
					default:
						break;
				}
				break;

			} else if (j+1 == LEN_ARGS){
				logging(ERROR, "Unknown arg error\n");
				return 1;
			}
		}
	}
	return 0;
}

//used by load config to apply parameters in file
int apply_parameter(char* key, char* value){
	const char key_arr[3][16] = {
		"port", "verbose", " "	
	};

	int i = 0;
	while(key_arr[i][0] != ' '){
		if(strcmp(key, key_arr[i]) == 0){

			switch(i){
				case 0: //port
					if(port == -1){
						logging(LOG, "Loading parameter %s to %s\n", key, value);
						port = atoi(value);
					}
					break;
				case 1: //verbose
					if(log_level == -1){
						logging(LOG, "Loading parameter %s to %s\n", key, value);
						log_level = atoi(value);
					}
					break;
				default:
					break;
			}
		}
		++i;	
	}
	return 0;
}

//load config.ini at config_path
int load_config(char* path){
	// Open the config file for reading
	logging(LOG, "Loading config file at %s\n", config_path);
	FILE *file = fopen(path, "r");
	if (file == NULL) {
		logging(ERROR, "Error opening config file at %s\n", config_path);
		return 1;
	}

	char line[64];
	char section[64];
	char key[64];
	char value[64];

	// Read the file line by line
	while (fgets(line, sizeof(line), file) != NULL) {
		// Trim leading and trailing whitespace
		char *trimmed_line = strtok(line, "\r\n");

		// Skip empty lines
		if (trimmed_line[0] == '\0')continue;

		// Check if this line represents a section
		if (trimmed_line[0] == '[') {
			// Extract section name
			sscanf(trimmed_line, "[%[^]]", section);
			logging(LOG, "Entering section %s\n", section);
		} else {
			// Parse key-value pairs
			sscanf(trimmed_line, "%[^=] = %[^\n]", key, value);
			//printf("reading %s:%s\n", key, value);
			if(apply_parameter(key, value))return 1;
		}
	}

	// Close the file
	fclose(file);
	logging(LOG, "Config file loaded\n");
	return 0;
}

//return DD-MM-YYYY for log file name
char* get_timestamp() {
	time_t now = time(NULL);
	static char time_str[20];
	strftime(
			time_str,
		       	sizeof(time_str),
		       	"%d-%m-%Y@%H:%M:%S",
		       	localtime(&now)
	);
	return time_str;
}

//log things, use like printf but with enum LOG_LEVEL as first arg
void logging(enum LOG_LEVEL level, const char* msg, ...){
	if(level > log_level)return;

	char full_msg[256] = {0};

	switch(level){
		case LOG:
			strncat(full_msg, "LOG : ", 7);
			break;
		case WARNING:
			strncat(full_msg, "WARNING : ", 10);
			break;
		case ERROR:
			strncat(full_msg, "ERROR : ", 9);
			break;
		default:
			break;

	}	


	//concat
	va_list args;
	va_start(args, msg);
	vsprintf(full_msg + strlen(full_msg), msg, args);

	//print msg
	printf("%s", full_msg);

	//log to file
	char* folder = "log/";

	//if dir not exist
	if(!opendir(folder)){
		//create it
		if (mkdir(folder, S_IRWXU | S_IRGRP | S_IXGRP | S_IROTH | S_IXOTH)){
			perror("Could not create log folder or can't access it\n");
			exit(1);
		}
		
	}

	char* name = get_timestamp();
	char* end = ".log";

	char filename[19] = {0};

	strncat(filename, folder, 4);
	strncat(filename + 4, name, 10);
	strncat(filename + 14, end, 5);

	FILE* log_file = fopen(filename, "a");

	fprintf(log_file, "%s", full_msg);

	fclose(log_file);
}

char* log_level_to_string(enum LOG_LEVEL level){
	switch(level){
		case NONE:
			return "NONE";
		case ERROR:
			return "ERROR";
		case WARNING:
			return "WARNING";
		case LOG:
			return "LOG";

	}
}

int create_master_sock(int port){
	int sock;
	/*
	 * AF_INET = IPV4
	 * SOCK_STREAM = IO_STREAM
	 * IPPROTO_TCP = TCP 
	 */

	sock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);	
	if(sock < 0){
		logging(ERROR, "Could not open socket\n");	
		return -1;
	}

	struct sockaddr_in server_addr;

	memset(&server_addr, 0, sizeof(server_addr));
	server_addr.sin_family = AF_INET;
	server_addr.sin_addr.s_addr = INADDR_ANY;
	server_addr.sin_port = htons(port); 

	struct sockaddr* addr = (struct sockaddr*)(&server_addr);
	socklen_t size = sizeof(server_addr);

	int binded = bind(sock, addr, size);

	if(binded < 0){
		logging(ERROR, "Could not bind socket\n");	
		return -1;
	}

	int listened = listen(sock, 10);

	if(listened < 0){
		logging(ERROR, "Could not listen socket\n");	
		return -1;
	}

	fcntl(sock, F_SETFL, O_NONBLOCK);

	return sock;
}

int process(int connection){
	char buff[16*1024] = {0};
	logging(LOG, "Thread started\n");

	int read = recv(connection, buff, 1024*16, 0);
	if(read < 0){
		logging(ERROR, "Could not read from socket\n");
		return 3;
	}

	logging(LOG, "< %s\n", buff);

	//TODO parse then process then answer

	close(connection);
	return 0;
}

void sigint_handler(int signum) {
	logging(LOG, "Ctrl+c received, exiting\n");
	running = 0;
}

int main(int argc, char** argv){

	if (signal(SIGINT, sigint_handler) == SIG_ERR) {
		logging(WARNING, "Cannot catch ctrl+c, exit will be dirty\n");
	}

	//parsing args
	if(parse_args(argc, argv))return 1;

	//loading config
	if(load_config(config_path))return 2;	

	//start
	logging(LOG, "--------------------------------------------------------\n");
	logging(LOG, "Starting on %s at %s:%d\n",
		       	get_timestamp(),
			"127.0.0.1",
			port
	);
	logging(LOG, "--------------------------------------------------------\n");

	int main_sock = create_master_sock(port);
	if(main_sock < 0)return 3;
		
	//connection var
	struct sockaddr_in server_addr;
	struct sockaddr* addr = (struct sockaddr*)(&server_addr);
	memset(&server_addr, 0, sizeof(server_addr));
	socklen_t size = sizeof(server_addr);
	int connection;

	//main boucle
	while(running){

		//peer data
		memset(&server_addr, 0, sizeof(server_addr));

		//waiting for connection
		connection = accept(main_sock, addr, &size);

		if(connection){
			//start thread to process client request
			process(connection);
		}
		usleep(100000);
	}

	close(main_sock);

	//end
	logging(LOG, "--------------------------------------------------------\n");
	logging(LOG, "Tracker stopped at %s\n", get_timestamp());
	logging(LOG, "--------------------------------------------------------\n");
	return 0;
};

