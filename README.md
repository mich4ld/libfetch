# fetch-os
Simple but reliable library for fetch tools.

## Overview
It is hard to create cool fetch info tool for Linux, so this library tries simplify that process.

Library is not calling any other programs like `uname -n` or `uptime`. It is trying to fetch OS
info in more reliable way.

## Goals
- avoid unsafe blocks (but not always possible);
- no external programs calling;
- multiplatform;
