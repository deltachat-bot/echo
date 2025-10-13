To run C# bot,
you need to install .NET.
On Arch Linux it can be done with
`pacman -S dotnet-sdk`.

To build the program,
run `dotnet build`.

To run the program,
run `dotnet run`.

To get debugging logs of all JSON-RPC messages, run with `RUST_LOG="deltachat_rpc_server=trace" dotnet run`.
