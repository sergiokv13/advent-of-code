defmodule DAY20 do

  def is_conj(state, node) do
    is_map(Map.get(state, node, false))
  end

  def parse_input() do
    {graph, state} = File.read!("inputs/day_20.input")
    |> String.split("\n")
    |> Enum.reduce({%{},%{}}, fn conn, {graph, state} ->
      [source, raw_dest] = String.split(conn, " -> ")
      dest = String.split(raw_dest, ", ")
      init_state = cond do
        String.contains?(source, "%") -> false
        String.contains?(source, "&") ->  %{}
        true -> false
      end

      source = String.replace(source, "%", "") |> String.replace("&", "")
      {
        Map.put(graph, source, dest),
        Map.put(state, source, init_state),
      }
    end)

    # update state &
    state = Map.to_list(graph)
    |> Enum.reduce(state, fn {parent, children}, acc_state ->

      Enum.filter(children, fn child -> is_conj(acc_state, child) end)
      |> Enum.reduce(acc_state, fn child, as ->
        current_state = Map.get(as, child)
        new_state = Map.put(current_state, parent, false)
        Map.put(as, child, new_state)
      end)

    end)

    {graph, state}
  end

  def get_signal(state, node) do
    curr_state = Map.get(state, node, false)
    cond do
      is_conj(state, node) ->
        if (Map.values(curr_state) |> Enum.all?(fn el -> el == true end)), do: false, else: true
      true -> curr_state
    end
  end

  def update_state(state, input_node, node, signal) do
    curr_state = Map.get(state, node, false)

    new_state = cond do
      is_conj(state, node) -> Map.put(state, node, Map.put(curr_state, input_node, signal))
      true -> Map.put(state, node, (if (signal), do: curr_state, else: !curr_state ))
    end

    new_state
  end

  def simulate_click(_, [], state, count, found), do: {state, count, found}
  def simulate_click(graph, queue, state, count, found) do
    [current | rest] = queue

    current_signal = get_signal(state, current)

    found = Enum.reduce(Map.keys(found), found, fn node, acc ->
      if (current == node && current_signal == true && Map.get(acc, node, false) == false) do
        Map.put(acc, node, true)
      else
        acc
      end
    end)

    {queue, state, count} = Map.get(graph, current, [])
    |> Enum.reduce({rest, state, count}, fn child, {acc_queue, acc_state, {count_pos, count_neg}} ->
      {acc_state, continue} = cond do
        current_signal == false -> { update_state(acc_state,current,child,false), true}
        current_signal == true && is_conj(state, child) -> { update_state(acc_state,current,child,true), true }
        true -> {acc_state, false}
      end

      count = if (current_signal) do
        {count_pos + 1, count_neg}
      else
        {count_pos, count_neg + 1}
      end

      {(if (continue), do: acc_queue ++ [child], else: acc_queue), acc_state, count}
    end)
    simulate_click(graph, queue, state, count, found)
  end

  def gcd(a, 0), do: a
  def gcd(a, b), do: gcd(b, rem(a, b))

  def lcm(a, b), do: div(abs(a * b), gcd(a, b))

  def lcm_list([x]), do: x
  def lcm_list([x | rest]), do: lcm(x, lcm_list(rest))

  def first_star() do
    {graph, state} = parse_input()

    {_, {high, low}, _} = Enum.reduce(1..1000, {state, {0,0}, false}, fn _, {state, count, _} ->
      simulate_click(graph, ["broadcaster"], state, count, %{})
    end)

    high * (low + 1000)
  end

  def second_star() do
    {graph, state} = parse_input()
    {_,_, found} = Enum.reduce(
      # Get for each of the conj inputs
      1..10000, {state, {0,0}, %{"ln" => 0, "db" => 0, "vq" => 0, "tf" => 0}},
      fn idx, {state, count, found} ->
        simulate_found = Map.to_list(found) |> Enum.into(%{}, fn {key, val} ->
          if (val == 0), do: {key, false}, else: {key, true}
        end)

        {ns, nc, nf} = simulate_click(graph, ["broadcaster"], state, count, simulate_found)

        acc_found = Map.to_list(found) |> Enum.into(%{}, fn {key, val} ->
          cond do
            val == 0 && Map.get(nf, key) == true -> {key, idx}
            true -> {key, val}
          end
        end)

        {ns, nc, acc_found}
      end)

    # found includes the steps in which we have the correct signal for the rx inputs
    # we get the lcm to get the value in which all will intercept
    found
    |> Map.values
    |> lcm_list
  end
end
