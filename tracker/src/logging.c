#include "logging.h"
#include "parameters.h"

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

//print help message (-v / --verbose)
void print_help(){
	const char* help_message = "Usage: tracker [OPTION...] [OPTION VALUE] \n\n\
	OPTION # OPTION VALUE # DECRIPTION \n\n\
	--verbose or -v # [0:4] (ERROR=0, WARNING (default), LOG, DEBUG, NONE # Sets verbose level \n\
	--help or -h # # Show this message \n\
	--config or -c # <path to config> # Sets path to config.ini \n\
	--max-conn or -m # [1:MAX_TASKS] # Set the number of simultaneous task processing \n\
	--port or -p # [1:65535] # Sets the tracker's listening port \n\
	--cache-time or -t # int # time to live in seconds of databse entry\n";


	printf("%s", help_message);
}

//return name of enum as string based on enum number
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
		case DEBUG:
			return "DEBUG";

	}
	return "UNSET";
}

//log things, use like printf but with enum LOG_LEVEL as first arg
void logging(enum LOG_LEVEL level, const char* msg, ...){
	if(level > log_level)return;
	if(log_level == NONE)return;

	char full_msg[1024*16] = {0};

	switch(level){
		case DEBUG:
			strcat(full_msg, "DEBUG : ");
			break;
		case LOG:
			strcat(full_msg, "LOG : ");
			break;
		case WARNING:
			strcat(full_msg, "WARNING : ");
			break;
		case ERROR:
			strcat(full_msg, "ERROR : ");
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
	DIR* exists = opendir(folder);
	if(!exists){
		//create it
		if (mkdir(folder, S_IRWXU | S_IRGRP | S_IXGRP | S_IROTH | S_IXOTH)){
			perror("Could not create log folder or can't access it\n");
			exit(1);
		}
		
	}

	closedir(exists);

	char* name = get_timestamp();
	char* end = ".log";

	char filename[19] = {0};

	strncat(filename, folder, 4);
	strncat(filename + 4, name, 10);
	strcat(filename + 14, end);

	FILE* log_file = fopen(filename, "a");

	if(log_file == NULL){
		printf("WARNING : Cannot open %s\n", filename);
		return;
	}

	fprintf(log_file, "%s", full_msg);

	fclose(log_file);
}

