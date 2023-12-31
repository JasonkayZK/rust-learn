# P2p Sync demo

Use append log to sync data between peers in a p2p network!

## **How to use**

### **Sync old data**

Start a peer:

```shell
cargo run

[ INFO]: rust_learn - Peer Id: 12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP
[ INFO]: rust_learn::dir - 数据文件创建成功
[ INFO]: libp2p_mdns::behaviour::iface - creating instance on iface 192.168.31.22
[ INFO]: rust_learn::swarm - Income swarm Event: NewListenAddr { listener_id: ListenerId(1), address: "/ip4/127.0.0.1/tcp/50950" }
[ INFO]: rust_learn::swarm - Income swarm Event: NewListenAddr { listener_id: ListenerId(1), address: "/ip4/192.168.31.22/tcp/50950" }
```

Insert some data:

```shell
create r 1|1|1

[ INFO]: rust_learn::handlers - Created recipe:
[ INFO]: rust_learn::handlers - Id: 7147228881180381185
[ INFO]: rust_learn::handlers - Name:  1
[ INFO]: rust_learn::handlers - Ingredients: 1
[ INFO]: rust_learn::handlers - Instructions:: 1
[ INFO]: rust_learn::handlers - Recipe create log appended: 7147228881180381185
[ INFO]: rust_learn::swarm::handler - incoming publish
[ WARN]: rust_learn::handlers - Recipe Insert opt broadcast err: InsufficientPeers

create r 2|2|2

[ INFO]: rust_learn::handlers - Created recipe:
[ INFO]: rust_learn::handlers - Id: 7147228881180381186
[ INFO]: rust_learn::handlers - Name:  2
[ INFO]: rust_learn::handlers - Ingredients: 2
[ INFO]: rust_learn::handlers - Instructions:: 2
[ INFO]: rust_learn::handlers - Recipe create log appended: 7147228881180381186
[ INFO]: rust_learn::swarm::handler - incoming publish
[ WARN]: rust_learn::handlers - Recipe Insert opt broadcast err: InsufficientPeers
```

Since there is only one peer right now, we can't broadcast the current operation to the other peers. 

Start another peer:

```shell
cargo run

[ INFO]: rust_learn - Peer Id: 12D3KooWDnCSmYtn1joJdVtgajuurJkHyp6PP1ZSWuuSeVGauWkr
[ INFO]: rust_learn::dir - 数据文件创建成功
[ INFO]: libp2p_mdns::behaviour::iface - creating instance on iface 192.168.31.22
[ INFO]: rust_learn::swarm - Income swarm Event: NewListenAddr { listener_id: ListenerId(1), address: "/ip4/127.0.0.1/tcp/50974" }
[ INFO]: rust_learn::swarm - Income swarm Event: NewListenAddr { listener_id: ListenerId(1), address: "/ip4/192.168.31.22/tcp/50974" }
[ INFO]: libp2p_mdns::behaviour - discovered: 12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP /ip4/192.168.31.22/tcp/50950
[ INFO]: rust_learn::swarm - Income swarm Event: Behaviour(Mdns(Discovered([(PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP"), "/ip4/192.168.31.22/tcp/50950")])))
[ INFO]: rust_learn::swarm - Income swarm Event: Dialing { peer_id: Some(PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP")), connection_id: ConnectionId(1) }
[ INFO]: rust_learn::swarm - Income swarm Event: IncomingConnection { connection_id: ConnectionId(2), local_addr: "/ip4/192.168.31.22/tcp/50974", send_back_addr: "/ip4/192.168.31.22/tcp/50975" }
[ INFO]: rust_learn::swarm - Income swarm Event: ConnectionEstablished { peer_id: PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP"), connection_id: ConnectionId(1), endpoint: Dialer { address: "/ip4/192.168.31.22/tcp/50950/p2p/12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 7.302917ms }
[ INFO]: rust_learn::swarm - [Connection established] peer_id: 12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP, connection_id: 1, endpoint: Dialer { address: "/ip4/192.168.31.22/tcp/50950/p2p/12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP", role_override: Dialer }, num_established: 1
[ INFO]: rust_learn::swarm - Income swarm Event: ConnectionEstablished { peer_id: PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP"), connection_id: ConnectionId(2), endpoint: Listener { local_addr: "/ip4/192.168.31.22/tcp/50974", send_back_addr: "/ip4/192.168.31.22/tcp/50975" }, num_established: 2, concurrent_dial_errors: None, established_in: 6.988417ms }
[ INFO]: rust_learn::swarm - [Connection established] peer_id: 12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP, connection_id: 2, endpoint: Listener { local_addr: "/ip4/192.168.31.22/tcp/50974", send_back_addr: "/ip4/192.168.31.22/tcp/50975" }, num_established: 2
[ INFO]: rust_learn::swarm - Income swarm Event: Behaviour(Gossip(Subscribed { peer_id: PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP"), topic: TopicHash { hash: "recipes" } }))
[ INFO]: rust_learn::swarm - Income swarm Event: Behaviour(Gossip(Subscribed { peer_id: PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP"), topic: TopicHash { hash: "broadcast-opt" } }))
[ INFO]: rust_learn::swarm - Income swarm Event: Behaviour(Gossip(Subscribed { peer_id: PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP"), topic: TopicHash { hash: "init-sync" } }))
[ INFO]: rust_learn::swarm::handler - incoming publish
[ INFO]: rust_learn::swarm - Income swarm Event: Behaviour(Gossip(Message { propagation_source: PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP"), message_id: MessageId(313244334b6f6f57527039576952426e634a3168476e77314b75657a476576314b554a59574a65796a6546646f67357237337a5031373034303332313134313333323038303033), message: Message { data: 7b2263757272656e74.., source: Some(PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP")), sequence_number: Some(1704032114133208003), topic: TopicHash { hash: "init-sync" } } }))
[ INFO]: rust_learn::swarm::gossip_event - Got swarm message: Message { data: 7b2263757272656e74.., source: Some(PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP")), sequence_number: Some(1704032114133208003), topic: TopicHash { hash: "init-sync" } }
[ INFO]: rust_learn::swarm - Income swarm Event: Behaviour(Gossip(Subscribed { peer_id: PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP"), topic: TopicHash { hash: "sync-12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP-12D3KooWDnCSmYtn1joJdVtgajuurJkHyp6PP1ZSWuuSeVGauWkr" } }))
[ INFO]: rust_learn::swarm - Income swarm Event: Behaviour(Gossip(Subscribed { peer_id: PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP"), topic: TopicHash { hash: "sync-12D3KooWDnCSmYtn1joJdVtgajuurJkHyp6PP1ZSWuuSeVGauWkr-12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP" } }))
[ INFO]: rust_learn::sync::progress_manager - Sending sync data: topic: sync-12D3KooWDnCSmYtn1joJdVtgajuurJkHyp6PP1ZSWuuSeVGauWkr-12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP, indexes: []
[ INFO]: rust_learn::swarm::handler - incoming publish

[ERROR]: rust_learn - unknown command: ""
[ INFO]: rust_learn::sync::progress_manager - Send sync data successfully!
[ INFO]: rust_learn::swarm - Income swarm Event: Behaviour(Gossip(Message { propagation_source: PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP"), message_id: MessageId(313244334b6f6f57527039576952426e634a3168476e77314b75657a476576314b554a59574a65796a6546646f67357237337a5031373034303332313134313333323038303035), message: Message { data: 7b227265636970655f.., source: Some(PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP")), sequence_number: Some(1704032114133208005), topic: TopicHash { hash: "sync-12D3KooWDnCSmYtn1joJdVtgajuurJkHyp6PP1ZSWuuSeVGauWkr-12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP" } } }))
[ INFO]: rust_learn::swarm::gossip_event - Got swarm message: Message { data: 7b227265636970655f.., source: Some(PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP")), sequence_number: Some(1704032114133208005), topic: TopicHash { hash: "sync-12D3KooWDnCSmYtn1joJdVtgajuurJkHyp6PP1ZSWuuSeVGauWkr-12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP" } }
[ INFO]: rust_learn::swarm::handler - incoming publish
[ INFO]: rust_learn::swarm - Income swarm Event: Behaviour(Gossip(Message { propagation_source: PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP"), message_id: MessageId(313244334b6f6f57527039576952426e634a3168476e77314b75657a476576314b554a59574a65796a6546646f67357237337a5031373034303332313134313333323038303034), message: Message { data: 7b226c6f6773223a5b.., source: Some(PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP")), sequence_number: Some(1704032114133208004), topic: TopicHash { hash: "sync-12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP-12D3KooWDnCSmYtn1joJdVtgajuurJkHyp6PP1ZSWuuSeVGauWkr" } } }))
[ INFO]: rust_learn::swarm::gossip_event - Got swarm message: Message { data: 7b226c6f6773223a5b.., source: Some(PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP")), sequence_number: Some(1704032114133208004), topic: TopicHash { hash: "sync-12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP-12D3KooWDnCSmYtn1joJdVtgajuurJkHyp6PP1ZSWuuSeVGauWkr" } }
[ INFO]: rust_learn::swarm::handler - incoming publish
[ INFO]: rust_learn::swarm - Income swarm Event: Behaviour(Gossip(Message { propagation_source: PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP"), message_id: MessageId(313244334b6f6f57527039576952426e634a3168476e77314b75657a476576314b554a59574a65796a6546646f67357237337a5031373034303332313134313333323038303036), message: Message { data: 7b2272656369706573.., source: Some(PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP")), sequence_number: Some(1704032114133208006), topic: TopicHash { hash: "sync-12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP-12D3KooWDnCSmYtn1joJdVtgajuurJkHyp6PP1ZSWuuSeVGauWkr" } } }))
[ INFO]: rust_learn::swarm::gossip_event - Got swarm message: Message { data: 7b2272656369706573.., source: Some(PeerId("12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP")), sequence_number: Some(1704032114133208006), topic: TopicHash { hash: "sync-12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP-12D3KooWDnCSmYtn1joJdVtgajuurJkHyp6PP1ZSWuuSeVGauWkr" } }
[ WARN]: rust_learn::sync::progress_manager - [set_sync_progress] topic: 12D3KooWRp9WiRBncJ1hGnw1KuezGev1KUJYWJeyjeFdog5r73zP, value: Vec([0, 1])
[ WARN]: rust_learn::sync::progress_manager - Set progress start: SyncProgress { bitmap: RoaringTreemap<[0, 1]> }
[ WARN]: rust_learn::sync::progress_manager - Set progress finished!
```

> Because of the `tokio:select!` halt, a `Enter` should be input.

The data will be synced:

```shell
ls r

[ INFO]: rust_learn::handlers - Local Recipes (2)
[ INFO]: rust_learn::handlers - (7147228881180381186, Recipe { id: 7147228881180381186, name: " 2", ingredients: "2", instructions: "2", shared: false })
[ INFO]: rust_learn::handlers - (7147228881180381185, Recipe { id: 7147228881180381185, name: " 1", ingredients: "1", instructions: "1", shared: false })
```

### **Sync new data**

Add a new entry:

```shell
create r new|new|new

[ INFO]: rust_learn::handlers - Created recipe:
[ INFO]: rust_learn::handlers - Id: 7147228881180381187
[ INFO]: rust_learn::handlers - Name:  new
[ INFO]: rust_learn::handlers - Ingredients: new
[ INFO]: rust_learn::handlers - Instructions:: new
[ INFO]: rust_learn::handlers - Recipe create log appended: 7147228881180381187
[ INFO]: rust_learn::swarm::handler - incoming publish
[ INFO]: rust_learn::handlers - Recipe Insert opt broadcast!
```

The operation has been broadcast!

And the operation has been synced:

```shell
ls r

[ INFO]: rust_learn::handlers - Local Recipes (3)
[ INFO]: rust_learn::handlers - (7147228881180381185, Recipe { id: 7147228881180381185, name: " 1", ingredients: "1", instructions: "1", shared: false })
[ INFO]: rust_learn::handlers - (7147228881180381187, Recipe { id: 7147228881180381187, name: " new", ingredients: "new", instructions: "new", shared: false })
[ INFO]: rust_learn::handlers - (7147228881180381186, Recipe { id: 7147228881180381186, name: " 2", ingredients: "2", instructions: "2", shared: false })
```

Try update:

```shell
publish r 7147228881180381187

[ INFO]: rust_learn::handlers - Published Recipe with id: 7147228881180381187
[ INFO]: rust_learn::handlers - Recipe update log append begin: 7147228881180381187->7147228881180381188
[ INFO]: rust_learn::handlers - Recipe Update log appended!
[ INFO]: rust_learn::swarm::handler - incoming publish
[ INFO]: rust_learn::handlers - Recipe Update opt broadcast!

# Other peer
ls r

[ INFO]: rust_learn::handlers - Local Recipes (3)
[ INFO]: rust_learn::handlers - (7147228881180381185, Recipe { id: 7147228881180381185, name: " 1", ingredients: "1", instructions: "1", shared: false })
[ INFO]: rust_learn::handlers - (7147228881180381186, Recipe { id: 7147228881180381186, name: " 2", ingredients: "2", instructions: "2", shared: false })
[ INFO]: rust_learn::handlers - (7147228881180381188, Recipe { id: 7147228881180381188, name: " new", ingredients: "new", instructions: "new", shared: true })
```

Delete:

```shell
delete r 7147228881180381188

[ INFO]: rust_learn::handlers - Deleted Recipe with id: 7147228881180381188
[ INFO]: rust_learn::handlers - Recipe delete log appended: 7147228881180381188
[ INFO]: rust_learn::swarm::handler - incoming publish
[ INFO]: rust_learn::handlers - Recipe Delete opt broadcast!

ls r

[ INFO]: rust_learn::handlers - Local Recipes (2)
[ INFO]: rust_learn::handlers - (7147228881180381185, Recipe { id: 7147228881180381185, name: " 1", ingredients: "1", instructions: "1", shared: false })
[ INFO]: rust_learn::handlers - (7147228881180381186, Recipe { id: 7147228881180381186, name: " 2", ingredients: "2", instructions: "2", shared: false })
```


# **Reference**

Blog:

- [《使用AppendLog和Gossip在P2P网络中同步状态》](https://jasonkayzk.github.io/2023/12/31/使用AppendLog和Gossip在P2P网络中同步状态/)
