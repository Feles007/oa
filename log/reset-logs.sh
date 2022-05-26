#!/bin/bash

# Compresses and clears logs

tar cfj "$(date '+%Y-%m-%d_%H-%M-%S').tar.bz2" access.log error.log

rm access.log
rm error.log

touch access.log
touch error.log
