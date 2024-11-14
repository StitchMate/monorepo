#!/usr/bin/env bash
echo STABLE_GIT_COMMIT $(git rev-parse HEAD)
echo RANDOM $(cat /dev/urandom | LC_ALL=C tr -dc 'a-zA-Z0-9' | fold -w 13 | head -n 1)
