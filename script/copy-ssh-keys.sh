#!/bin/bash
USER=root
PI_IP=192.168.0.99

echo "PRI $PI_IP"
ssh-copy-id $USER@$PI_IP
