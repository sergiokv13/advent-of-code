defmodule DAY10 do


  def print_mat(elements, {size_y, size_x}) do
    Enum.each(Range.new(0, size_y-1), fn y ->
      Enum.each(Range.new(0, size_x-1), fn x ->
        IO.write(Map.get(elements, {y,x}))
      end)
      IO.puts("")
    end)
  end

  def is_valid_neighboor(el, {size_y, size_x}) do
    case el do
      {-1, _} -> false
      {_, -1} -> false
      {_, ^size_x} -> false
      {^size_y, _} -> false
      _ -> true
    end
  end

  def get_neighboors({y,x}, el, {size_y, size_x}) do
    case el do
      "." -> []
      "|" -> [{y+1,x}, {y-1,x}] |> Enum.filter(&is_valid_neighboor(&1, {size_y, size_x}))
      "-" -> [{y,x-1}, {y,x+1}] |> Enum.filter(&is_valid_neighboor(&1, {size_y, size_x}))
      "L" -> [{y-1,x}, {y,x+1}] |> Enum.filter(&is_valid_neighboor(&1, {size_y, size_x}))
      "J" -> [{y-1,x}, {y,x-1}] |> Enum.filter(&is_valid_neighboor(&1, {size_y, size_x}))
      "7" -> [{y,x-1}, {y+1,x}] |> Enum.filter(&is_valid_neighboor(&1, {size_y, size_x}))
      "F" -> [{y+1,x}, {y,x+1}] |> Enum.filter(&is_valid_neighboor(&1, {size_y, size_x}))
    end
  end

  def build_graph(s_el) do
    elements = File.read!("inputs/day_10.input")
    |> String.split("\n")
    |> Enum.with_index
    |> Enum.flat_map(fn {row, y} ->
      String.graphemes(row)
        |> Enum.with_index
        |> Enum.map(fn {el,x} -> {{y, x}, el} end)
      end
    )
    {s_pos, _} = Enum.find(elements, fn {_, el} -> el == "S" end)

    size_y = File.read!("inputs/day_10.input") |> String.split("\n") |> Enum.count
    size_x = File.read!("inputs/day_10.input") |> String.split("\n") |> List.first |> String.graphemes |> Enum.count

    elements = Enum.map(elements, fn {pos, el} -> if el == "S" do {pos, s_el} else {pos, el} end end)

    raw_graph = Enum.into(elements, %{}, fn {pos, el} ->
        {pos, get_neighboors(pos, el, {size_y, size_x})}
    end)

    # Filter valid connections
    graph = Map.to_list(raw_graph)
    |> Enum.map(fn {k, v} ->
      { k, v |> Enum.filter(fn child -> k in Map.get(raw_graph, child) end)}
    end)
    |> Enum.into(%{})

    elements = Enum.into(elements, %{})
    { graph, s_pos, elements, {size_y, size_x} }
  end

  def bfs(graph, [], visited, init_pos) do
    all_childs = Map.get(graph, init_pos) |> Enum.all?(fn el -> el in visited end)
    n_childs = Enum.count(Map.get(graph, init_pos))
    { visited, all_childs && n_childs >= 2 }
  end

  def bfs(graph, queue, visited, init_pos) do
    [current_node | rest] = queue
    visited = MapSet.put(visited, current_node)
    valid_children = Map.get(graph, current_node)
                    |> Enum.reject(fn child -> child in visited end)

    visited = MapSet.union(visited, MapSet.new(valid_children))
    bfs(graph, rest ++ valid_children, visited, init_pos)
  end

  def get_direction(dir, el) do
    cond do
      el == "|" -> dir
      el == "-" -> dir

      el == "L" && dir == "B" -> "R"
      el == "L" && dir == "L" -> "U"
      el == "L" && dir == "R" -> "R"

      el == "J" && dir == "B" -> "L"
      el == "J" && dir == "R" -> "U"
      el == "J" && dir == "L" -> "L"

      el == "7" && dir == "R" -> "B"
      el == "7" && dir == "U" -> "L"
      el == "7" && dir == "L" -> "L"

      el == "F" && dir == "U" -> "R"
      el == "F" && dir == "L" -> "B"
      el == "F" && dir == "R" -> "R"
    end
  end


  def update_by_method(elements, pos, move_method, loop) do
    if Map.get(elements, pos) == nil || pos in loop do elements
    else update_by_method(Map.put(elements, pos, "X"), move_method.(pos), move_method, loop)
    end
  end

  def update_on_right({y,x}, direction, elements, loop) do
    case direction do
      "U" -> update_by_method(elements, {y,x+1}, fn {y,x} -> {y,x+1} end, loop)
      "R" -> update_by_method(elements, {y+1,x}, fn {y,x} -> {y+1,x} end, loop)
      "L"-> update_by_method(elements, {y-1,x}, fn {y,x} -> {y-1,x} end, loop)
      "B"-> update_by_method(elements, {y,x-1}, fn {y,x} -> {y,x-1} end, loop)
    end
  end

  def discard_out(_, elements, [], _, _), do: elements
  def discard_out(graph, elements, queue, visited, loop) do
    [{current_node, parentdirection, direction} | rest] = queue
    elements = update_on_right(current_node, direction, elements, loop)
    elements = update_on_right(current_node, parentdirection, elements, loop)

    visited = MapSet.put(visited, current_node)
    valid_children = Map.get(graph, current_node)
                    |> Enum.reject(fn child -> child in visited end)

    visited = MapSet.union(visited, MapSet.new(valid_children))

    valid_children  = valid_children |> Enum.map(fn child ->
      {child, direction, get_direction(direction, Map.get(elements, child)) }
    end)

    discard_out(graph, elements, rest ++ valid_children, visited, loop)
  end

  def first_star() do
    ["|", "-", "L", "J", "7", "F"]
    |> Enum.map(fn pipe -> {build_graph(pipe), pipe} end)
    |> Enum.map(fn {{graph, init_pos, _, _}, pipe} -> {bfs(graph, [init_pos], MapSet.new(), init_pos), pipe} end)
    |> Enum.filter(fn {{_, completed}, _} -> completed end )
    |> Enum.map(fn {{visited, _}, pipe} -> {:math.floor(Enum.count(visited) / 2), pipe, visited} end)
    |> List.first
  end

  def second_star() do
    {_, pipe, visited} = first_star()
    {graph, _, elements, _ } = build_graph(pipe)

    {_, min_x} = Enum.min_by(visited, fn {_,x} -> x end)
    {y,x} = Enum.filter(visited, fn {_,x} -> x == min_x end) |> Enum.min_by(fn {y,_} -> y end)
    {y,x}

    graph = Map.put(graph, {y,x}, [{y, x+1}])
    discard_out(graph, elements, [{{y,x}, "U", "R"}], MapSet.new(), visited)
    |> Map.to_list
    |> Enum.count(fn {_, el} -> el == "X" end)
  end
end
