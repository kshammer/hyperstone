# hyperstone

proto2 

First 4 bytes are the demo header, a dota 2 source 2 demos header should be "PBDEMS2" 
The next 8 bytes (4th to 12th byte) are a little endian byte string that tells you where to seek to find the CDemoFileInfo proto.
The next 4 bytes are for something else 
Start at byte 16 for the actual data.   