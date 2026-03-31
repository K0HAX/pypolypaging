#!/usr/bin/env python3
import pypolypaging
import time

def send_udp(alert, end, payload):
    import socket
    MCAST_GRP = '224.0.1.116'
    MCAST_PORT = 5001
    MULTICAST_TTL = 2
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM, socket.IPPROTO_UDP)
    sock.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_TTL, MULTICAST_TTL)
    for _ in range(32):
        sock.sendto(bytes(alert), (MCAST_GRP, MCAST_PORT))
        time.sleep(0.030)
    for p in payload:
        sock.sendto(bytes(p), (MCAST_GRP, MCAST_PORT))
        time.sleep(0.010)
    for _ in range(13):
        sock.sendto(bytes(end), (MCAST_GRP, MCAST_PORT))
        time.sleep(0.030)

def main():
    session = pypolypaging.SessionInfo(49, 0, 'Michael')
    alert = pypolypaging.get_alert(session)
    file = None
    with open('Doorbell.g722', 'rb') as f:
        file = f.read()
    codec = pypolypaging.CodecFlag.G722
    alert = pypolypaging.get_alert(session)
    end_packet = pypolypaging.get_end(session)
    payload = pypolypaging.get_payload_packets(session, codec, 0, file)
    #print(payload[0])
    #print(payload[1])
    #print(payload)
    send_udp(alert, end_packet, payload)

if __name__ == "__main__":
    main()

