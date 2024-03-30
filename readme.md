# How to use


- `cd server`
- `cargo run`
- Open few new terminals    
    - in each terminal: 
        - `cd client`
        - `cargo run`


The output should be:   

1. When new client is connected to server, server will broadcast message that connected client has sent (broadcast to all currently connected) and it will send a special message only to that just connected client.   
