# P2p Demo

A Branch to show how to use [libp2p](https://github.com/libp2p/rust-libp2p/).

## **How to use**

Start multiple p2p nodes:

```shell
cargo run

INFO  rust_learn > Peer Id: 12D3KooWA7xhiEmFxikn9aiWcffkhDACDhz1rRPXxkC4yxgnzJCT
INFO  libp2p_mdns::behaviour::iface > creating instance on iface 192.168.31.22
INFO  rust_learn::handlers          > Income swarm Event: NewListenAddr { listener_id: ListenerId(1), address: "/ip4/127.0.0.1/tcp/65248" }
INFO  rust_learn::handlers          > Income swarm Event: NewListenAddr { listener_id: ListenerId(1), address: "/ip4/192.168.31.22/tcp/65248" }
INFO  libp2p_mdns::behaviour        > discovered: 12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7 /ip4/192.168.31.22/tcp/65247
INFO  rust_learn::handlers          > Income swarm Event: Behaviour(Mdns(Discovered([(PeerId("12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7"), "/ip4/192.168.31.22/tcp/65247")])))
INFO  rust_learn::handlers          > Income swarm Event: Dialing { peer_id: Some(PeerId("12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7")), connection_id: ConnectionId(1) }
INFO  rust_learn::handlers          > Income swarm Event: ConnectionEstablished { peer_id: PeerId("12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7"), connection_id: ConnectionId(1), endpoint: Dialer { address: "/ip4/192.168.31.22/tcp/65247/p2p/12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 7.355625ms }
INFO  rust_learn::handlers          > Income swarm Event: Behaviour(Floodsub(Subscribed { peer_id: PeerId("12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7"), topic: Topic("recipes") }))
INFO  libp2p_mdns::behaviour        > discovered: 12D3KooWCWesVZsAoDaFs7UZYXV6gTNd56UPMoiWfxWFgvLCJZhz /ip4/192.168.31.22/tcp/65250
INFO  rust_learn::handlers          > Income swarm Event: Behaviour(Mdns(Discovered([(PeerId("12D3KooWCWesVZsAoDaFs7UZYXV6gTNd56UPMoiWfxWFgvLCJZhz"), "/ip4/192.168.31.22/tcp/65250")])))
INFO  rust_learn::handlers          > Income swarm Event: Dialing { peer_id: Some(PeerId("12D3KooWCWesVZsAoDaFs7UZYXV6gTNd56UPMoiWfxWFgvLCJZhz")), connection_id: ConnectionId(2) }
INFO  rust_learn::handlers          > Income swarm Event: IncomingConnection { connection_id: ConnectionId(3), local_addr: "/ip4/192.168.31.22/tcp/65248", send_back_addr: "/ip4/192.168.31.22/tcp/65253" }
INFO  rust_learn::handlers          > Income swarm Event: ConnectionEstablished { peer_id: PeerId("12D3KooWCWesVZsAoDaFs7UZYXV6gTNd56UPMoiWfxWFgvLCJZhz"), connection_id: ConnectionId(2), endpoint: Dialer { address: "/ip4/192.168.31.22/tcp/65250/p2p/12D3KooWCWesVZsAoDaFs7UZYXV6gTNd56UPMoiWfxWFgvLCJZhz", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 5.762334ms }
INFO  rust_learn::handlers          > Income swarm Event: ConnectionEstablished { peer_id: PeerId("12D3KooWCWesVZsAoDaFs7UZYXV6gTNd56UPMoiWfxWFgvLCJZhz"), connection_id: ConnectionId(3), endpoint: Listener { local_addr: "/ip4/192.168.31.22/tcp/65248", send_back_addr: "/ip4/192.168.31.22/tcp/65253" }, num_established: 2, concurrent_dial_errors: None, established_in: 5.212125ms }
INFO  rust_learn::handlers          > Income swarm Event: Behaviour(Floodsub(Subscribed { peer_id: PeerId("12D3KooWCWesVZsAoDaFs7UZYXV6gTNd56UPMoiWfxWFgvLCJZhz"), topic: Topic("recipes") }))
```

Show all peers:

```shell
ls p

 INFO  rust_learn::handlers          > Discovered Peers:
 INFO  rust_learn::handlers          > 12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7
 INFO  rust_learn::handlers          > 12D3KooWCWesVZsAoDaFs7UZYXV6gTNd56UPMoiWfxWFgvLCJZhz
```

Create Recipe:

```shell
create r name|recipe_ingredients|recipe_instruction

 INFO  rust_learn::handlers          > Created recipe:
 INFO  rust_learn::handlers          > Name:  name
 INFO  rust_learn::handlers          > Ingredients: recipe_ingredients
 INFO  rust_learn::handlers          > Instructions:: recipe_instruction

```

List local recipes:

```shell
ls r

 INFO  rust_learn::handlers          > Local Recipes (6)
 INFO  rust_learn::handlers          > Recipe { id: 0, name: " Coffee", ingredients: "Coffee", instructions: "Make Coffee", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 1, name: " Tea", ingredients: "Tea, Water", instructions: "Boil Water, add tea", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 2, name: " Carrot Cake", ingredients: "Carrots, Cake", instructions: "Make Carrot Cake", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 3, name: " Name", ingredients: "Ingredients", instructions: "Instructions", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 4, name: " name", ingredients: "recipeIngredients", instructions: "instruction", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 5, name: " name", ingredients: "recipe_ingredients", instructions: "recipe_instruction", shared: false }
```

List all remote recipes:

```shell
ls r all

 INFO  rust_learn::handlers          > Income swarm Event: Behaviour(Floodsub(Message(FloodsubMessage { source: PeerId("12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7"), data: [123, 34, 109, 111,...
 INFO  rust_learn::handlers          > Response from 12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7:
 INFO  rust_learn::handlers          > Recipe { id: 0, name: " Coffee", ingredients: "Coffee", instructions: "Make Coffee", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 1, name: " Tea", ingredients: "Tea, Water", instructions: "Boil Water, add tea", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 2, name: " Carrot Cake", ingredients: "Carrots, Cake", instructions: "Make Carrot Cake", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 3, name: " Name", ingredients: "Ingredients", instructions: "Instructions", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 4, name: " name", ingredients: "recipeIngredients", instructions: "instruction", shared: true }
 INFO  rust_learn::handlers          > Income swarm Event: Behaviour(Floodsub(Message(FloodsubMessage { source: PeerId("12D3KooWCWesVZsAoDaFs7UZYXV6gTNd56UPMoiWfxWFgvLCJZhz"), data: [123, 34, 109....
 INFO  rust_learn::handlers          > Response from 12D3KooWCWesVZsAoDaFs7UZYXV6gTNd56UPMoiWfxWFgvLCJZhz:
 INFO  rust_learn::handlers          > Recipe { id: 0, name: " Coffee", ingredients: "Coffee", instructions: "Make Coffee", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 1, name: " Tea", ingredients: "Tea, Water", instructions: "Boil Water, add tea", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 2, name: " Carrot Cake", ingredients: "Carrots, Cake", instructions: "Make Carrot Cake", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 3, name: " Name", ingredients: "Ingredients", instructions: "Instructions", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 4, name: " name", ingredients: "recipeIngredients", instructions: "instruction", shared: true }
```

List corresponding peer's recipes:

```shell
ls r 12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7

 INFO  rust_learn::handlers          > Income swarm Event: Behaviour(Floodsub(Message(FloodsubMessage { source: PeerId("12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7"), data: [123, 34, 109, ...
 INFO  rust_learn::handlers          > Response from 12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7:
 INFO  rust_learn::handlers          > Recipe { id: 0, name: " Coffee", ingredients: "Coffee", instructions: "Make Coffee", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 1, name: " Tea", ingredients: "Tea, Water", instructions: "Boil Water, add tea", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 2, name: " Carrot Cake", ingredients: "Carrots, Cake", instructions: "Make Carrot Cake", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 3, name: " Name", ingredients: "Ingredients", instructions: "Instructions", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 4, name: " name", ingredients: "recipeIngredients", instructions: "instruction", shared: true }
```

You could see that unshared recipes are not shown!

Publish recipe:

```shell
publish r 5

 INFO  rust_learn::handlers          > Published Recipe with id: 5
```

List remote recipes again:

```shell
ls r 12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7

 INFO  rust_learn::handlers          > Income swarm Event: Behaviour(Floodsub(Message(FloodsubMessage { source: PeerId("12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7"), data: [123, 34, 109, ...
 INFO  rust_learn::handlers          > Response from 12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7:
 INFO  rust_learn::handlers          > Recipe { id: 0, name: " Coffee", ingredients: "Coffee", instructions: "Make Coffee", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 1, name: " Tea", ingredients: "Tea, Water", instructions: "Boil Water, add tea", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 2, name: " Carrot Cake", ingredients: "Carrots, Cake", instructions: "Make Carrot Cake", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 3, name: " Name", ingredients: "Ingredients", instructions: "Instructions", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 4, name: " name", ingredients: "recipeIngredients", instructions: "instruction", shared: true }
 INFO  rust_learn::handlers          > Recipe { id: 5, name: " name", ingredients: "recipe_ingredients", instructions: "recipe_instruction", shared: true }
```

As you can see the recipe that we just published shown!


## **Reference**

Blog:

- [《Rust中使用libp2p》](https://jasonkayzk.github.io/2023/12/27/Rust中使用libp2p/)

