
{:ok, a} = Ockam.Transport.UDP.create port: 3000

Ockam.Transport.UDP.send a, %Ockam.Message{
  onward_route: [{:udp, {{127,0,0,1}, 4000}}, "80cc2379"],
  payload: "hello"
}
