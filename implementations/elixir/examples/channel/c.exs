
Ockam.Transport.UDP.create port: 5000

{:ok, vault} = Ockam.Vault.create()
{:ok, identity_keypair} = Ockam.Vault.generate_curve25519_keypair(vault)

Ockam.Channel.create [role: :responder, s: identity_keypair, vault: vault]


{:ok, responder_printer} = Ockam.Worker.create fn(message) ->
  IO.puts("responder - *** #{inspect message} ***")
end
