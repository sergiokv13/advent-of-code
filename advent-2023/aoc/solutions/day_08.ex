defmodule DAY08 do
  def parse_input() do
    [instructions, _ | tail ] = File.read!("inputs/day_08.input")
    |> String.replace(")", "")
    |> String.replace("(", "")
    |> String.split("\n")

    graph = Enum.into(tail, %{}, fn row ->
      [parent, children] = String.split(row, " = ")
      {parent, String.split(children, ", ")}
    end)

    {String.graphemes(instructions), graph}
  end

  def repeat_ultil_z([curr_instruction | instructions], graph, curr_node, steps, condition) do
    if condition.(curr_node) do
      steps
    else
      [left, right] = Map.get(graph, curr_node)

      new_node = if curr_instruction == "L" do left else right end
      repeat_ultil_z(instructions ++ [curr_instruction], graph, new_node, steps + 1, condition)
    end
  end

  def first_star() do
    { instructions, graph } = parse_input()
    repeat_ultil_z(instructions, graph, "AAA", 0, fn node -> node == "ZZZ" end)
  end

  def gcd(a, 0), do: a
  def gcd(a, b), do: gcd(b, rem(a, b))
  def lcm(a, b), do: div(abs(a * b), gcd(a, b))

  def second_star() do
    { instructions, graph } = parse_input()
    nodes_with_a = Map.keys(graph) |> Enum.filter(fn node -> String.ends_with?(node, "A") end)
    nodes_with_z = Map.keys(graph) |> Enum.filter(fn node -> String.ends_with?(node, "Z") end)

    Enum.map(nodes_with_a, fn node ->
      repeat_ultil_z(instructions, graph, node, 0, fn cnode -> cnode in nodes_with_z end)
    end)
    |> Enum.reduce(1, fn el, acc -> lcm(el, acc) end)
  end
end


# recompile
# DAY08.second_star()
