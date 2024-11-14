#!/usr/bin/env bash
QUERY="kind('oci_load',//...)"
BUILD_TARGETS=$(bazel query "$QUERY")
bazel run "$BUILD_TARGETS"
