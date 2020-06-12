
{:ok, printer} =
  Ockam.Worker.create fn(message) ->
    IO.puts "Got Message: #{inspect message}"
  end

{:ok, b} = Ockam.Transport.UDP.create port: 4000
