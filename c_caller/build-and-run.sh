#!/usr/bin/env bash

set -xeE

bazel build :all
./bazel-bin/c-caller

