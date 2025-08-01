#!/usr/bin/env bash

set -e +o pipefail


show_help() {
cat << EOF

      gitig Gitignore CLI

    -h          Display this help and exit

    -V VERSION  Custom Software version. Default is 'latest'

EOF
}

abort() {
  printf "%s\n" "$@" >&2
  exit 1
}

# First check OS.
OS="$(uname)"
if [[ "${OS}" == "Linux" ]]
then
  CLI_ON_LINUX=1
elif [[ "${OS}" == "Darwin" ]]
then
  CLI_ON_MACOS=1
else
  abort "Currently is only supported on macOS and Linux."
fi

VERSION="latest"

if [ $# != 0 ];
then
  while getopts "hV:-" o
  do
    case "$o" in
      "h")
        show_help
        exit 0;
        ;;
      "V")
        VERSION=v"$OPTARG"
        ;;
      *)
        echo -e "Unexpected flag not supported"
        exit 1
        ;;
    esac
  done
fi

VERSION="latest"


echo -e "
..............888888888888888888888 ........=8888888888888888888D=..............
...........88888888888888888888888 ..........D8888888888888888888888I...........
.........,8888888888888ZI: ...........................=Z88D8888888888D..........
.........+88888888 ..........................................88888888D..........
.........+88888888 .......Welcome to use Gitignore CLI.......O8888888D..........
.........+88888888 ..... https://github.com/cfanbo/gitig ....O8888888D..........
.........+88888888 .... Command Line Interface(Reloaded) ....O8888888D..........
.........+88888888...........................................88888888D..........
..........D888888888888DO+. ..........................?ND888888888888D..........
...........O8888888888888888888888...........D8888888888888888888888=...........
............ .:D8888888888888888888.........78888888888888888888O ..............
"

if [[ -n "${CLI_ON_MACOS-}" ]]
then
  UNAME_MACHINE="$(/usr/bin/uname -m)"
  if [[ "${UNAME_MACHINE}" == "arm64" || "${UNAME_MACHINE}" == "aarch64" ]]
  then
    curl -O -fsSL https://githubfiles.oss-cn-shanghai.aliyuncs.com/gitig/gitig-"$VERSION"-aarch64-apple-darwin.tar.gz
    tar zxf gitig-"$VERSION"-aarch64-apple-darwin.tar.gz
  else
    curl -O -fsSL https://githubfiles.oss-cn-shanghai.aliyuncs.com/gitig/gitig-"$VERSION"-x86_64-apple-darwin.tar.gz
    tar zxf gitig-"$VERSION"-x86_64-apple-darwin.tar.gz
  fi
  mv ./gitig /usr/local/bin/
fi

if [[ -n "${CLI_ON_LINUX-}" ]]
then
  UNAME_MACHINE="$(/usr/bin/uname -m)"
  if [[ "${UNAME_MACHINE}" == "arm64" || "${UNAME_MACHINE}" == "aarch64" ]]
  then
    curl -O -fsSL https://githubfiles.oss-cn-shanghai.aliyuncs.com/gitig/gitig-"$VERSION"-aarch64-unknown-linux-gnu.tar.gz
    tar zxf gitig-"$VERSION"-aarch64-unknown-linux-gnu.tar.gz
  else
    curl -O -fsSL https://githubfiles.oss-cn-shanghai.aliyuncs.com/gitig/gitig-"$VERSION"-x86_64-unknown-linux-gnu.tar.gz
    tar zxf gitig-"$VERSION"-x86_64-unknown-linux-gnu.tar.gz
  fi
  mv ./gitig /usr/local/bin/
fi