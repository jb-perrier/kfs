#!/bin/bash

if [ -d ./bin ]; then
    rm -rf ./bin/*
fi

if [ -d ./build ]; then
    rm -rf ./build/*
fi

if [ -d ./kernel/target ]; then
    rm -rf ./kernel/target/*
fi