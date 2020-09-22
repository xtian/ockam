
{args, _, _} = System.argv |> OptionParser.parse(strict: [
  route_to_channel_responder: :string,
  worker_address: :string
])
{route, _} = Keyword.get(args, :route_to_channel_responder) |> Code.eval_string
worker_address = Keyword.get(args, :worker_address)

{:ok, _} = Ockam.Transport.UDP.create port: 3000

{:ok, v} = Ockam.Vault.create()
{:ok, kp} = Ockam.Vault.generate_curve25519_keypair(v)

{:ok, c} = Ockam.Channel.create(role: :initiator, vault: v, identity_keypair: kp, onward_route: route)


{:ok, forwarder} =
  Ockam.Worker.create fn({:udp, _, _, _, packet} = m) ->
    IO.inspect m
    Ockam.Router.route %Ockam.Message{payload: packet, onward_route: [c.address, worker_address]}
  end

{:ok, socket} = :gen_udp.open(7700, [:binary, :inet, {:ip, {127,0,0,1}}, {:active, true}])
:gen_udp.controlling_process(socket, Ockam.Worker.whereis(forwarder))
