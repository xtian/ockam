
{args, _, _} = System.argv |> OptionParser.parse(strict: [forward_to_port: :integer])
foward_to_port = Keyword.get(args, :forward_to_port)

{:ok, _} = Ockam.Transport.UDP.create port: 5000

{:ok, v} = Ockam.Vault.create()
{:ok, kp} = Ockam.Vault.generate_curve25519_keypair(v)

{:ok, c} = Ockam.Channel.create role: :responder, identity_keypair: kp, vault: v
IO.puts "Channel Responder Address: #{c.external_address}"

{:ok, forwarder} =
  Ockam.Worker.create fn(m) ->
    {:ok, socket} = :gen_udp.open(7701, [:binary, :inet, {:ip, {127,0,0,1}}, {:active, true}])
    :gen_udp.send(socket, {127,0,0,1}, foward_to_port, m.payload)
    :gen_udp.close(socket)
  end
IO.puts "Forwarder Address: #{forwarder.address}"
