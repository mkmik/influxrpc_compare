# influxrpc_compare
Comparison tool for influx storage gRPC logs



## Example useage:

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
