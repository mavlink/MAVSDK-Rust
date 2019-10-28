#!/usr/bin/env bash

set -e

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

pb_plugins_dir="${script_dir}/../proto/pb_plugins"
proto_dir="${script_dir}/../proto/protos"
backend_generated_dir="${script_dir}/../src/generated"
protoc_binary=protoc

plugin_list="action calibration geofence gimbal camera core info mission offboard param telemetry mocap"

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

echo ""
echo "-------------------------------"
echo "Generating the SDK wrappers"
echo "-------------------------------"
echo ""

if [ ! -d ${pb_plugins_dir}/venv ]; then
    python3 -m venv ${pb_plugins_dir}/venv

    source ${pb_plugins_dir}/venv/bin/activate
    pip install -r ${pb_plugins_dir}/requirements.txt
    pip install -e ${pb_plugins_dir}
fi

source ${pb_plugins_dir}/venv/bin/activate

for plugin in ${plugin_list}; do
    ${protoc_binary} -I ${proto_dir} --plugin=protoc-gen-custom=`which protoc-gen-dcsdk` --custom_out=${backend_generated_dir} --custom_opt=file_ext=rs,template_path=${script_dir}/../templates ${proto_dir}/${plugin}/${plugin}.proto
done
