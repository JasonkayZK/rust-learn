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

 INFO  rust_learn::handlers          > Income swarm Event: Behaviour(Floodsub(Message(FloodsubMessage { source: PeerId("12D3KooWGEGJQhFaR4ZzJ15CUvMVVu1wcaGd3i7yzvHHYFexbfT7"), data: [123, 34, 109, 111, 100, 101, 34, 58, 34, 65, 108, 108, 34, 44, 34, 100, 97, 116, 97, 34, 58, 91, 123, 34, 105, 100, 34, 58, 48, 44, 34, 110, 97, 109, 101, 34, 58, 34, 32, 67, 111, 102, 102, 101, 101, 34, 44, 34, 105, 110, 103, 114, 101, 100, 105, 101, 110, 116, 115, 34, 58, 34, 67, 111, 102, 102, 101, 101, 34, 44, 34, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 34, 58, 34, 77, 97, 107, 101, 32, 67, 111, 102, 102, 101, 101, 34, 44, 34, 115, 104, 97, 114, 101, 100, 34, 58, 116, 114, 117, 101, 125, 44, 123, 34, 105, 100, 34, 58, 49, 44, 34, 110, 97, 109, 101, 34, 58, 34, 32, 84, 101, 97, 34, 44, 34, 105, 110, 103, 114, 101, 100, 105, 101, 110, 116, 115, 34, 58, 34, 84, 101, 97, 44, 32, 87, 97, 116, 101, 114, 34, 44, 34, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 34, 58, 34, 66, 111, 105, 108, 32, 87, 97, 116, 101, 114, 44, 32, 97, 100, 100, 32, 116, 101, 97, 34, 44, 34, 115, 104, 97, 114, 101, 100, 34, 58, 116, 114, 117, 101, 125, 44, 123, 34, 105, 100, 34, 58, 50, 44, 34, 110, 97, 109, 101, 34, 58, 34, 32, 67, 97, 114, 114, 111, 116, 32, 67, 97, 107, 101, 34, 44, 34, 105, 110, 103, 114, 101, 100, 105, 101, 110, 116, 115, 34, 58, 34, 67, 97, 114, 114, 111, 116, 115, 44, 32, 67, 97, 107, 101, 34, 44, 34, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 34, 58, 34, 77, 97, 107, 101, 32, 67, 97, 114, 114, 111, 116, 32, 67, 97, 107, 101, 34, 44, 34, 115, 104, 97, 114, 101, 100, 34, 58, 116, 114, 117, 101, 125, 44, 123, 34, 105, 100, 34, 58, 51, 44, 34, 110, 97, 109, 101, 34, 58, 34, 32, 78, 97, 109, 101, 34, 44, 34, 105, 110, 103, 114, 101, 100, 105, 101, 110, 116, 115, 34, 58, 34, 73, 110, 103, 114, 101, 100, 105, 101, 110, 116, 115, 34, 44, 34, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 34, 58, 34, 73, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 34, 44, 34, 115, 104, 97, 114, 101, 100, 34, 58, 116, 114, 117, 101, 125, 44, 123, 34, 105, 100, 34, 58, 52, 44, 34, 110, 97, 109, 101, 34, 58, 34, 32, 110, 97, 109, 101, 34, 44, 34, 105, 110, 103, 114, 101, 100, 105, 101, 110, 116, 115, 34, 58, 34, 114, 101, 99, 105, 112, 101, 73, 110, 103, 114, 101, 100, 105, 101, 110, 116, 115, 34, 44, 34, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 34, 58, 34, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 34, 44, 34, 115, 104, 97, 114, 101, 100, 34, 58, 116, 114, 117, 101, 125, 93, 44, 34, 114, 101, 99, 101, 105, 118, 101, 114, 34, 58, 34, 49, 50, 68, 51, 75, 111, 111, 87, 65, 55, 120, 104, 105, 69, 109, 70, 120, 105, 107, 110, 57, 97, 105, 87, 99, 102, 102, 107, 104, 68, 65, 67, 68, 104, 122, 49, 114, 82, 80, 88, 120, 107, 67, 52, 121, 120, 103, 110, 122, 74, 67, 84, 34, 125], sequence_number: [77, 178, 79, 49, 236, 159, 21, 8, 133, 223, 91, 164, 228, 8, 160, 163, 64, 103, 218, 221], topics: [Topic("recipes")] })))
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

