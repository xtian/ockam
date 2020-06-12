
{:ok, vault} = Ockam.Vault.create()

{:ok, keypair} = Ockam.Vault.generate_curve25519_keypair(vault)
