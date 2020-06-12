
Ockam.Transport.UDP.create port: 3000

{:ok, vault} = Ockam.Vault.create()
{:ok, identity_keypair} = Ockam.Vault.generate_curve25519_keypair(vault)

{:ok, printer} = Ockam.Worker.create fn(message) ->
  IO.puts("initiator - *** #{inspect message} ***")
end



Ockam.Channel.create(
  role: :initiator,
  s: identity_keypair,
  vault: vault,
  onward_route: [
    {:udp, {{127,0,0,1}, 4000}},
    {:udp, {{127,0,0,1}, 5000}},
    "f2737577"
  ],
  route_incoming_messages_to: [printer.address]
)


Ockam.Router.route "fb93810f", {:plaintext, %Ockam.Message{payload: "hello"}}
