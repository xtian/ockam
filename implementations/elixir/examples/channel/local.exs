{:ok, initiator_vault} = Ockam.Vault.create()

{:ok, initiator_identity_keypair} = Ockam.Vault.generate_curve25519_keypair(initiator_vault)

{:ok, initiator_printer} = Ockam.Worker.create fn(message) ->
  IO.puts("initiator - *** #{inspect message} ***")
end




{:ok, responder_vault} = Ockam.Vault.create()

{:ok, responder_identity_keypair} = Ockam.Vault.generate_curve25519_keypair(responder_vault)

{:ok, responder_printer} = Ockam.Worker.create fn(message) ->
  IO.puts("responder - *** #{inspect message} ***")
end




Ockam.Channel.create [role: :responder, s: responder_identity_keypair, vault: responder_vault, route_incoming_messages_to: [responder_printer.address]]


Ockam.Channel.create role: :initiator, s: initiator_identity_keypair, vault: initiator_vault, onward_route: [
  "a48f78b2"
], route_incoming_messages_to: [initiator_printer.address]


Ockam.Router.route "a48f78b2", {:plaintext, %Ockam.Message{payload: "hello"}}
