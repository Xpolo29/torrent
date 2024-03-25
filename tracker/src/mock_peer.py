import socket
import hashlib
import json
import random

# Tracker address and port
TRACKER_ADDRESS = "localhost"
TRACKER_PORT = 7878


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


def update_tracker(update):
    message = f"{update}\n"
    return send_message(message)


# Example usage
if __name__ == "__main__":
    # Announce files to tracker
    hash1 = calculate_hash()
    hash2 = calculate_hash()
    while hash2 == hash1:
        hash2 = calculate_hash()

    files = f"[file_a.dat 1024 16 {hash1} file_a.dat 4096 32 {hash2}]"
    announce_response = announce_files(TRACKER_PORT, files)
    print("Announce response:", announce_response)

    # Look for files on tracker
    criteria = "[filename='file_a.dat']"
    look_response = look_for_files(criteria)
    print("Look response:", look_response)

    criteria = "[filename='file_a.dat' filesize='1024']"
    look_response = look_for_files(criteria)
    print("Look response:", look_response)

    # Getfile
    peers_response = get_peers(hash1)
    print("Peers response:", peers_response)

    # Update
    update = f"update seed [{hash2}] leech []"
    update_response = update_tracker(update)
    print("Update reponse:", update_response)

    # Retry look knowing its gone now
    criteria = '[filename="file_a.dat" filesize<"2048"]'
    look_response = look_for_files(criteria)
    print("Look response:", look_response)

    # Getfile 2, file is gone
    peers_response = get_peers(hash1)
    print("Peers response:", peers_response)
