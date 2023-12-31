#!/bin/bash

echo "Crashc project setup wizard"

check_approve() {
  echo -n "Would you like to continue? [y/n]: "
  read -r input

  if [ "$input" == "y" ]; then
    echo "Continuing setup."
    return
  fi

  if [ "$input" == "n" ]; then
    echo "Cancelling setup."
    exit 0
  fi

  echo "Invalid input!"

  check_approve
}

check_sudo() {
  sudo echo "Checking sudo mode."

  if sudo -n true 2>/dev/null; then
    echo "Sudo granted."
    return
  fi

  echo "No sudo mode active. Please retry again."
  exit 1
}

check_requirements() {
  echo "Checking for LLVM, CMake, zLib, Python, GNU Make."

  if [ "$(uname)" == "Darwin" ]; then
    echo "Detected macOS. Installing Homebrew."
    if command -v brew &>/dev/null; then
      echo "Homebrew package manager detected."
      sudo brew update
      sudo brew install llvm llvm-devel cmake zlib python make
    fi
    return
  fi

  if command -v apt-get &>/dev/null; then
    echo "APT package manager detected."
    sudo apt-get update
    sudo apt-get install llvm llvm-devel zlib1g-dev python make cmake -y
  elif command -v dnf &>/dev/null; then
    echo "DNF package manager detected."
    sudo dnf install llvm llvm-devel zlib-devel python make cmake -y
  elif command -v yum &>/dev/null; then
    echo "YUM package manager detected."
    sudo yum install llvm llvm-devel zlib-devel python make cmake -y
  elif command -v pacman &>/dev/null; then
    echo "PACMAN package manager detected."
    sudo pacman -Syu --noconfirm llvm llvm-devel cmake zlib python make
  elif command -v zypper &>/dev/null; then
    echo "Zypper package manager detected."
    sudo zypper --non-interactive install llvm llvm-devel zlib-devel python make
  else
    echo "No supported package manager detected. Unable to setup automatically."
    exit 1
  fi

  echo "Installed LLVM."
}

check_approve
check_sudo
check_requirements