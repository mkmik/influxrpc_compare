# influxrpc_compare
Comparison tool for gRPC binary [logs] containing the requests /
responses in influxdb storage gRPC [format]. These files are typically
named something like `grpcgo_binarylog_2709101216.txt`



## Example dumping combined gRPC calls:
```shell
cargo run -- dump-calls dump-calls --path  ~/Documents/grpc_comparison

Attempting to dump "/Users/alamb/Documents/grpc_comparison/grpcgo_binarylog_2709101216.txt"
Read 3212922 bytes from "/Users/alamb/Documents/grpc_comparison/grpcgo_binarylog_2709101216.txt"
Read 43435 ok entries and 1 err entries in 227.505945msFound 8691 calls
Call(id=     1) [2022-02-10 19:40:55.025444431 UTC-2022-02-10 19:40:55.026975206 UTC] /influxdata.platform.storage.Storage/Capabilities storage-0.storage.twodotoh-dev-alamb.svc:8082 --> 10.84.12.170:8082
Call(id=     2) [2022-02-10 19:40:55.026325162 UTC-2022-02-10 19:40:55.026983140 UTC] /influxdata.platform.storage.Storage/Capabilities storage-1.storage.twodotoh-dev-alamb.svc:8082 --> 10.84.55.164:8082
Call(id=     3) [2022-02-10 19:40:55.027157243 UTC-2022-02-10 19:40:55.027547431 UTC] /influxdata.platform.storage.Storage/Offsets storage-1.storage.twodotoh-dev-alamb.svc:8082 --> 10.84.55.164:8082
Call(id=     4) [2022-02-10 19:40:55.027177066 UTC-2022-02-10 19:40:55.027658055 UTC] /influxdata.platform.storage.Storage/Offsets storage-0.storage.twodotoh-dev-alamb.svc:8082 --> 10.84.12.170:8082
Call(id=     5) [2022-02-10 19:40:56.020766956 UTC-2022-02-10 19:40:56.021438681 UTC] /influxdata.platform.storage.Storage/Offsets storage-1.storage.twodotoh-dev-alamb.svc:8082 --> 10.84.55.164:8082
Call(id=     6) [2022-02-10 19:40:56.020767028 UTC-2022-02-10 19:40:56.021422538 UTC] /influxdata.platform.storage.Storage/Offsets storage-0.storage.twodotoh-dev-alamb.svc:8082 --> 10.84.12.170:8082
Call(id=     7) [2022-02-10 19:40:57.020854946 UTC-2022-02-10 19:40:57.021509828 UTC] /influxdata.platform.storage.Storage/Offsets storage-1.storage.twodotoh-dev-alamb.svc:8082 --> 10.84.55.164:8082

...

Non storage offset call:
  Call(id=   133) [2022-02-10 19:41:55.986380152 UTC-2022-02-10 19:41:56.036413945 UTC] /influxdata.platform.storage.Storage/TagValues storage-1.storage.twodotoh-dev-alamb.svc:8082 --> 10.84.55.164:8082
  request: Some(TagValuesRequest(TagValuesRequest { tags_source: Some(Any { type_url: "type.googleapis.com/com.github.influxdata.idpe.storage.read.ReadSource", value: b"\x08\xb4\x8d\xd1\xce\xe2\xc6\xf3\xbd\xee\x01\x10\xeb\xfe\x87\xc6\x9c\xb6\xa8\xb2k\x18\x01" }), range: Some(TimestampRange { start: 1644518515968401801, end: 1644522115968401801 }), predicate: None, tag_key: [108, 111, 99, 97, 116, 105, 111, 110] }))
  response: Some(StringValuesResponse(StringValuesResponse { values: [[99, 111, 121, 111, 116, 101, 95, 99, 114, 101, 101, 107], [115, 97, 110, 116, 97, 95, 109, 111, 110, 105, 99, 97]] }))
```

## Example dumping raw gRPC entries:

```shell
cargo run
cargo run -- dump-entries --path  ~/Documents/grpc_comparison

Attempting to dump "/Users/alamb/Documents/grpc_comparison/grpcgo_binarylog_2709101216.txt"
Read 3212922 bytes from "/Users/alamb/Documents/grpc_comparison/grpcgo_binarylog_2709101216.txt"
Entry {
    timestamp: Some(
        2022-02-10T19:40:55.025444431Z,
    ),
    call_id: 1,
    sequence_id_within_call: 1,
    event_type: ClientHeader,
    logger: Client,
    payload_truncated: false,
    peer: None,
    payload: ClientHeader(
        ClientHeader {
            metadata: {
                "uber-trace-id": "54909f0553f3f2a0:54909f0553f3f2a0:0000000000000000:0",
            },
            method_name: "/influxdata.platform.storage.Storage/Capabilities",
            authority: "storage-0.storage.twodotoh-dev-alamb.svc:8082",
            timeout: None,
        },
    ),
}
Entry {
    timestamp: Some(
        2022-02-10T19:40:55.025996142Z,
    ),
    call_id: 1,
    sequence_id_within_call: 2,
    event_type: ClientMessage,
    logger: Client,
    payload_truncated: false,
    peer: None,
    payload: Message(
        Message {
            length: 0,
            data: "<..0 bytes..>",
        },
    ),
}
Entry {
    timestamp: Some(
        2022-02-10T19:40:55.026325162Z,
    ),
    call_id: 2,
    sequence_id_within_call: 1,
    event_type: ClientHeader,
    logger: Client,
    payload_truncated: false,
    peer: None,
    payload: ClientHeader(
        ClientHeader {
            metadata: {
                "uber-trace-id": "2e5ff01a137a35bf:2e5ff01a137a35bf:0000000000000000:0",
            },
            method_name: "/influxdata.platform.storage.Storage/Capabilities",
            authority: "storage-1.storage.twodotoh-dev-alamb.svc:8082",
            timeout: None,
        },
    ),
}
...
```

[logs]: https://github.com/grpc/proposal/blob/master/A16-binary-logging.md
[format]: https://github.com/grpc/grpc-proto/blob/master/grpc/binlog/v1/binarylog.proto
