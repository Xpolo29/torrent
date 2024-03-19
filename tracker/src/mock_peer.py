import socket
import hashlib
import json
import random
import threading

# Tracker address and port
TRACKER_ADDRESS = "localhost"
TRACKER_PORT = 12345


# Function to calculate MD5 hash
def calculate_hash():
    random_data = str(random.randint(0, 99999999999999999999999999999999999999999999999999999999999999999999999999999999999)).encode()
    hash_md5 = hashlib.md5(random_data)
    return hash_md5.hexdigest()


# Function to send and receive messages to/from tracker
def send_message(message):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((TRACKER_ADDRESS, TRACKER_PORT))
        s.sendall(message.encode())
        data = s.recv(1024)
    return data.decode()


# Function to announce presence and files to tracker
def announce_files(port, files):
    message = f"announce listen {port} seed {json.dumps(files)[1:-1]} leech []\r\n"
    return send_message(message)


# Function to look for files on tracker
def look_for_files(criteria):
    message = f"look {json.dumps(criteria)[1:-1]}\r\n"
    return send_message(message)


# Function to get peers for a file from tracker
def get_peers(file_key):
    message = f"getfile {file_key}\r\n"
    return send_message(message)


def send_id(x):
    i = 1
    while(i <= 4):
        send_message(x + "(" + str(i) + ")")
        i += 1


# Example usage
if __name__ == "__main__":
    #
    #L = []
    #for i in range(100):
    #    thread = threading.Thread(target=send_id, args=(str(i),))
    #    thread.start()
    #    L.append(thread)
    #
    # L[-1].join()
    #exit()

    # Announce files to tracker
    port = 12345

    files = f"[file_a.dat 2097152 1024 {calculate_hash()}]"
    announce_response = announce_files(port, files)
    print("Announce response:", announce_response)

    # Look for files on tracker
    criteria = "[filename='file_a.dat' filesize='1048576']"
    look_response = look_for_files(criteria)
    print("Look response:", look_response)

    # Get peers for a file from tracker
    file_key = calculate_hash()
    peers_response = get_peers(file_key)
    print("Peers response:", peers_response)

