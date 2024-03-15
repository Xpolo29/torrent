import socket
import hashlib
import json
import random

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
    message = f"< announce listen {port} seed {json.dumps(files)} leech []\r\n"
    return send_message(message)


# Function to look for files on tracker
def look_for_files(criteria):
    message = f"< look {json.dumps(criteria)}\r\n"
    return send_message(message)


# Function to get peers for a file from tracker
def get_peers(file_key):
    message = f"< getfile {file_key}\r\n"
    return send_message(message)


# Example usage
if __name__ == "__main__":
    # Announce files to tracker
    port = 12345
    files = [
        {"filename": "file_a.dat", "filesize": 2097152, "piecesize": 1024, "key": calculate_hash()},
        {"filename": "file_b.dat", "filesize": 3145728, "piecesize": 1536, "key": calculate_hash()}
    ]
    announce_response = announce_files(port, files)
    print("Announce response:", announce_response)

    # Look for files on tracker
    criteria = {"filename": "file_a.dat", "filesize": 1048576}
    look_response = look_for_files(criteria)
    print("Look response:", look_response)

    # Get peers for a file from tracker
    file_key = calculate_hash()
    peers_response = get_peers(file_key)
    print("Peers response:", peers_response)

