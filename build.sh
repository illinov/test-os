#!/bin/bash

while getopts "m:b:u:" opt; do
  case $opt in
    m) MODE="multiboot2" ;;
    b) MODE="bios" ;;
    u) MODE="uefi" ;;
    *) echo "Invalid option: -$OPTARG" >&2 ;;
  esac
done

if 