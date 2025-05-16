#!/bin/sh
set -e

echo "Starting the application..."

exec /usr/local/bin/ems-backend "$@"
