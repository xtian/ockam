
# A worker is a process that invokes a supplied function whenever it receives
# a new message.
#
# Workers may be `stateless` or `statful`.
#
# A simple stateful worker can be created by passing an anonymous function
# of arity 2 to `Ockam.Worker.create` along with the initial state.

# Here we create a worker, called added, that has initial state 100 and adds
# every new message to this state. It assumes that every incoming message will
# be a number.

{:ok, adder} =
  Ockam.Worker.create(100, fn message, sum ->
    sum = sum + message
    IO.puts "Sum is: #{sum}"
    {:ok, sum}
  end)

# Lets send adder some messages.

Ockam.Worker.send adder, 300
Ockam.Worker.send adder, 500
Ockam.Worker.send adder, 2300
Ockam.Worker.send adder, 600

# Here we create another worker that simply collects all messages it receives
# in a list and prints the current collection whenever it receives a new
# message.

{:ok, collector} =
  Ockam.Worker.create [], fn(message, collection) ->
    collection = [message | collection]
    IO.puts "Collection is: #{inspect collection}"
    {:ok, collection}
  end

Ockam.Worker.send collector, "a"
Ockam.Worker.send collector, "b"
Ockam.Worker.send collector, "c"
Ockam.Worker.send collector, "d"

# Here's another worker that forwards every message it receives to a different
# worker, the collector that we created above.

{:ok, forwarder} =
  Ockam.Worker.create collector, fn(message, next) ->
    Ockam.Worker.send next, message
    {:ok, next}
  end

Ockam.Worker.send forwarder, "x"
Ockam.Worker.send forwarder, "y"
Ockam.Worker.send forwarder, "z"
