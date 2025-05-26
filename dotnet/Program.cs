using System;
using System.Diagnostics;
using System.ComponentModel;
using System.Text.Json.Serialization;
using System.Text.Json;
using StreamJsonRpc;
using System.Buffers;

Process childProcess = Process.Start(new ProcessStartInfo("deltachat-rpc-server")
{
    RedirectStandardInput = true,
    RedirectStandardOutput = true,
});

JsonRpc rpc = new JsonRpc(new StreamJsonRpc.NewLineDelimitedMessageHandler(childProcess.StandardInput.BaseStream, childProcess.StandardOutput.BaseStream, new StreamJsonRpc.SystemTextJsonFormatter()));
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
  Event eventResponse = await rpc.InvokeAsync<Event>("get_next_event");
  if (eventResponse.@event is InfoEventType infoEvent) {
    Console.WriteLine($"INFO: {infoEvent.msg}");
  } else {
    Console.WriteLine($"Got event {eventResponse}");
  }
}

public class Event {
  public int contextId { get; set; }
  public EventType @event { get; set; }
}

[JsonPolymorphic(TypeDiscriminatorPropertyName = "kind", IgnoreUnrecognizedTypeDiscriminators = true)]
[JsonDerivedType(typeof(InfoEventType), typeDiscriminator: "Info")]
public class EventType {
}

public class InfoEventType : EventType {
  public string msg { get; set; }
}
