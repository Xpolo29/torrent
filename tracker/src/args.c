#include "args.h"
#include "logging.h"
#include "parameters.h"

//used by load config to apply parameters in file
int apply_parameter(char* key, char* value){
	const char key_arr[5][16] = {
		"port", "verbose", "max-conn", "cache-time", " "	
	};

	int i = 0;
	while(key_arr[i][0] != ' '){
		if(strcmp(key, key_arr[i]) == 0){

			switch(i){
				case 0: //port
					if(port == -1){
						logging(DEBUG, "Loading parameter %s to %s\n", key, value);
						port = atoi(value);
					}
					break;
				case 1: //verbose
					if(log_level == UNSET){
						logging(DEBUG, "Loading parameter %s to %s\n", key, value);
						log_level = atoi(value);
					}
					break;
				case 2: //max-conn
					if( thread_pool_size == -1){
						logging(DEBUG, "Loading parameter %s to %s\n", key, value);
						thread_pool_size = atoi(value);
					}
					break;
				case 3: //cache-time
					if(time_to_live == -1){
						logging(DEBUG, "Loading parameter %s to %s\n", key, value);
						time_to_live = atoi(value);
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
			logging(DEBUG, "Entering section %s\n", section);
		} else {
			// Parse key-value pairs
			sscanf(trimmed_line, "%[^=] = %[^\n]", key, value);
			//printf("reading %s:%s\n", key, value);
			if(apply_parameter(key, value))return 1;
		}
	}

	// Close the file
	fclose(file);
	logging(DEBUG, "Config file loaded\n");
	return 0;
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
							log_level = atoi(argv[i + 1]) % (NONE + 1);
							++i;
						}else{
							if(log_level == UNSET){log_level = 2;}
							else{log_level =(log_level + 1) % (NONE + 1);}
						}
						break;
					case 2: // -h
					case 3: // --help
						print_help();
						exit(0);
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
					case 8: // -m
					case 9: // --max-conn
						if(i + 1 < argc){
							thread_pool_size = atoi(argv[i + 1]);
							if(thread_pool_size < 1 || thread_pool_size > MAX_THREAD_POOL){
								logging(WARNING, "-m value is outside of bounds [1:%d], using default\n", MAX_THREAD_POOL);
								thread_pool_size = -1;
							} else 
								logging(LOG, "Max simultaneous connection updated to %d\n", thread_pool_size);
							++i;
						}else{
							logging(WARNING, "Got -m but no max connection is specified, using default\n");
						}
						break;
					case 10: // -t
					case 11: // --cache-time
						if(i + 1 < argc){
							time_to_live = atoi(argv[i + 1]);
							logging(LOG, "Cache lifespan updated to %d\n", time_to_live);
							++i;
						}else{
							logging(WARNING, "Got -t but no value is specified, using default\n");
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

