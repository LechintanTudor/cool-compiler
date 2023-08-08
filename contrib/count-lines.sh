#!/bin/sh

find -name '*.rs' | xargs wc -l | sort -n
