#!/usr/bin/env bash

bazel build :all
./bazel-bin/c-caller

