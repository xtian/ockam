
{:ok, printer} =
  Ockam.Worker.create fn(message) ->
    IO.puts "Got Message: #{inspect message}"
  end

Ockam.Router.register "bbbbbbbb", Ockam.Worker.whereis(printer)

{:ok, b} = Ockam.Transport.UDP.create port: 4000
