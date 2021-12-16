import fileinput
from typing import DefaultDict
import math 

hex_bin = { '0': '0000', '1': '0001', '2': '0010', '3': '0011', '4': '0100', '5': '0101', '6': '0110', '7': '0111', '8': '1000', '9': '1001', 'A': '1010', 'B': '1011', 'C': '1100', 'D': '1101', 'E': '1110', 'F': '1111'}

def convert_to_bin(packet):
    return "".join([hex_bin[char] for char in packet])

def get_binpacket_version_type(binpacket):
    p_version = int(binpacket[:3], 2)
    p_type = int(binpacket[3:6], 2)
    return p_version, p_type

def get_value(values, ptype):
    if ptype == 0: return sum(values)
    if ptype == 1: return math.prod(values)
    if ptype == 2: return min(values)
    if ptype == 3: return max(values)
    if ptype == 5: return int(values[0] > values[1])
    if ptype == 6: return int(values[0] < values[1])
    if ptype == 7: return int(values[0] == values[1])

def decode_bin_packet(binpacket):
    p_version, p_type = get_binpacket_version_type(binpacket)
    if p_type == 4:
        # iterate on 5 bits groups
        init = 6
        bin_num = ""
        while True:
            closing_indicator = binpacket[init]
            bin_num += binpacket[init+1:init+5]
            init += 5
            if (closing_indicator == '0'):
                break
        return {
            "value": int(bin_num, 2),
            "trailing_data": binpacket[init:],
            "version_sum": p_version,
            "type": p_type
        }
    # operation packet
    else:
        type_id = binpacket[6]
        if type_id == '0':
            children_length = int(binpacket[7:22], 2)
            packet_data = binpacket[22:22+children_length]
            version_sum = p_version
            values = []
            while(packet_data):
                p_data = decode_bin_packet(packet_data)
                values.append(p_data["value"])
                version_sum += p_data["version_sum"]
                packet_data = p_data["trailing_data"]

            return {
                "value": get_value(values, p_type),
                "trailing_data": binpacket[22+children_length:],
                "version_sum": version_sum,
                "type": p_type
            }
        else:
            number_of_packages = int(binpacket[7:18], 2)
            packet_data = binpacket[18:]
            version_sum = p_version
            values = []
            for i in range(number_of_packages):
                p_data = decode_bin_packet(packet_data)
                values.append(p_data["value"])
                version_sum += p_data["version_sum"]
                packet_data = p_data["trailing_data"]
            return {
                "value": get_value(values, p_type),
                "trailing_data": packet_data,
                "version_sum": version_sum,
                "type": p_type
            }


def decode_packet(packet):
    binpacket = convert_to_bin(packet)
    return decode_bin_packet(binpacket)


packet = None
for line in fileinput.input():
    packet = line.strip()

decoded_packet = decode_packet(packet)

# First Star

print(decoded_packet["version_sum"])

# Second Star

print(decoded_packet["value"])