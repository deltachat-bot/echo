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

bool isConfigured = await rpc.InvokeAsync<bool>("is_configured", accountId);
if(!isConfigured) {
  Console.WriteLine("not configured");
  await rpc.InvokeAsync("set_config_from_qr", accountId, "dcaccount:https://nine.testrun.org/new");
  await rpc.InvokeAsync("configure", accountId);
}

await rpc.InvokeAsync("start_io_for_all_accounts");

while(true) {
  Event res2 = await rpc.InvokeAsync<Event>("get_next_event");
  Console.WriteLine("Got event {0} {1}", res2.contextId, res2.@event.kind);
}

public class Event {
  public int contextId;
  public EventType @event;
}

public class EventType {
  public string kind;
}
