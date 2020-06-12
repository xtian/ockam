
# A worker is a process that invokes a supplied function whenever it receives
# a new message.
#
# Workers may be `stateless` or `statful`.

# A simple stateless worker can be created by passing an anonymous function
# of arity 1 to `Ockam.Worker.create`

# Here we create a worker that prints every message it receives to standard
# output.

{:ok, printer} =
  Ockam.Worker.create fn(message) ->
    IO.puts "Message is: #{message}"
  end

# Now we can send messages to the worker using `Ockam.Worker.send`.
Ockam.Worker.send printer, "hello"
Ockam.Worker.send printer, "hi"
Ockam.Worker.send printer, "how are you?"

# Workers can also be created using the elixir `&` capture operator
# Here we capture the `IO.puts/1` so the worker prints every message it
# receives to standard output.

{:ok, printer} = Ockam.Worker.create &IO.puts/1
Ockam.Worker.send printer, "hello"
Ockam.Worker.send printer, "hi"
Ockam.Worker.send printer, "how are you?"

# We may create a worker by capturing `IO.inspect/1` instead to deal with
# messages that don't implement the String.Chars protocol.

{:ok, inspector} = Ockam.Worker.create &IO.inspect/1

Ockam.Worker.send inspector, "hello"
Ockam.Worker.send inspector, {a: 100}
