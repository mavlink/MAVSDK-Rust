#!/usr/bin/env sh

set -e

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

proto_dir="${script_dir}/src/proto/protos"
backend_generated_dir="${script_dir}/src/generated"
protoc_binary=protoc

plugin_list="action calibration geofence gimbal camera core info mission offboard param telemetry mocap shell"

echo ""
echo "-------------------------------"
echo " Generating pb and grpc.pb files"
echo "    * protoc --version: $(${protoc_binary} --version)"
echo "-------------------------------"
echo ""

mkdir -p ${backend_generated_dir}

for plugin in ${plugin_list}; do
  gdir=${backend_generated_dir}/${plugin}
  mkdir -p ${gdir}
  ${protoc_binary} -I ${proto_dir} --rust_out=${gdir} --grpc_out=${gdir} --plugin=protoc-gen-grpc=`which grpc_rust_plugin` ${proto_dir}/${plugin}/${plugin}.proto
done
