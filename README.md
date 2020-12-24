# rust-chat

This is a relatively simple implementation of a chat application written in Rust.
It uses the standard networking libraries built into Rust. The server listens on a TCP channel, and the client sends messages
from the user to the server. The server replies back with the same message sent to it by the client to acknowledge that it has received
the message the user was trying to send. Both the client and server use asynchronous channels and multithreading, so it is possible
to scale this up.
