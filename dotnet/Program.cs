using System;
using System.Diagnostics;
using System.ComponentModel;
using StreamJsonRpc;

Process childProcess = Process.Start(new ProcessStartInfo("deltachat-rpc-server")
{
    RedirectStandardInput = true,
    RedirectStandardOutput = true,
});

JsonRpc rpc = new JsonRpc(new StreamJsonRpc.NewLineDelimitedMessageHandler(childProcess.StandardInput.BaseStream, childProcess.StandardOutput.BaseStream, new StreamJsonRpc.JsonMessageFormatter()));
rpc.StartListening();

var accounts = await rpc.InvokeAsync<int[]>("get_all_account_ids");
int accountId;
if(accounts.Length == 0) {
  accountId = await rpc.InvokeAsync<int>("add_account");
} else {
  accountId = accounts[0];
}

var res = await rpc.InvokeAsync<Dictionary<String, String>>("get_system_info");

foreach(var item in res)
{
  Console.WriteLine("{0}={1}", item.Key, item.Value);
}
