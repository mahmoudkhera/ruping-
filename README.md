# udpopt

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)



**ruping** is a lightweight implementation of the classic `ping` command written entirely in **Rust** — from scratch  construct your ip and icmp packet by your hand

It sends ICMP echo requests to a target host and measures round-trip time (RTT), just like the standard `ping` utility.



##  Features



-  Sends and receives ICMP Echo Request/Reply packets  and may 
-  Measures RTT (round-trip time) accurately  
- Displays summary statistics on exit (`Ctrl + C`) 
- Implemented from scratch using system calls and raw sockets  


## Current limitations : 

- Supports both sync and async modes 
- No cli yet (hard coded destination)

- Contribution is appreciated to add the full ping features and cli



