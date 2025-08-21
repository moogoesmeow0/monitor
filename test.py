import socket
import time
import csv;

def append_to_csv(filename: str, data: list[tuple[int,int]], conn: socket.socket):
    try:
        with open(filename, 'a', newline='') as file:
            writer = csv.writer(file)
            for row in data:
                writer.writerow(row)
    except FileNotFoundError:
        with open(filename, 'w', newline='') as file:
            writer = csv.writer(file)
            for row in data:
                writer.writerow(row)

    conn.sendall(b"updated")


def connect() -> socket.socket:
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.bind(("127.0.0.1", 5000))
    print("socket bound")
    sock.listen(1)
    print("listening for connections")
    conn, _ = sock.accept()
    print("connection accepted")

    print(f"sending ping")
    conn.sendall(b"ping")

    return conn

def monitor():
    print("todo")
    raise NotImplementedError("Monitor function is not implemented yet")


if __name__ == "__main__":
    conn = connect()

    monitor()








    conn.close()



