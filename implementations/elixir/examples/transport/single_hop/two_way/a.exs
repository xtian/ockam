
{:ok, printer} =
  Ockam.Worker.create fn(message) ->
    IO.puts "Got Response: #{inspect message}"
  end

{:ok, a} = Ockam.Transport.UDP.create port: 3000

Ockam.Transport.UDP.send a, %Ockam.Message{
  onward_route: [{:udp, {{127,0,0,1}, 4000}}],
  return_route: [printer.address],
  payload: :ping
}
