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
}

//log things, use like printf but with enum LOG_LEVEL as first arg
void logging(enum LOG_LEVEL level, const char* msg, ...){
	if(level > log_level)return;

	char full_msg[1024*16] = {0};

	switch(level){
		case DEBUG:
			strncat(full_msg, "DEBUG : ", 9);
			break;
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
	strncat(filename + 14, end, 5);

	FILE* log_file = fopen(filename, "a");

	if(log_file == NULL){
		printf("WARNING : Cannot open %s\n", filename);
		return;
	}

	fprintf(log_file, "%s", full_msg);

	fclose(log_file);
}

