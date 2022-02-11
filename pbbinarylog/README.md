This direcotry contains compiled rust code for gRPC "binary log" format as described in https://github.com/grpc/proposal/blob/master/A16-binary-logging.md


Protobuf sources came from
* https://github.com/grpc/grpc-proto/blob/master/grpc/binlog/v1/binarylog.proto
* https://github.com/protocolbuffers/protobuf/tree/master/src/google/protobuf

To fetch latest:
```shell
wget https://raw.githubusercontent.com/grpc/grpc-proto/master/grpc/binlog/v1/binarylog.proto
wget https://raw.githubusercontent.com/protocolbuffers/protobuf/master/src/google/protobuf/duration.proto
wget https://raw.githubusercontent.com/protocolbuffers/protobuf/master/src/google/protobuf/timestamp.proto
```
