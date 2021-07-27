#!/bin/bash


namespaces=(constants formulas);
tmp_name="_tmp_proto.proto";

for ns in ${namespaces[@]}
do
  path="https://protobuf.graviton.one/$ns/$ns.proto";
  curl $path > $tmp_name;

  mv $tmp_name "./$ns.proto";
  protoc --rust_out . "./$ns.proto"
  rm "./$ns.proto";
  
  mv "./$ns.rs" src/proto
done

