defmodule DAY23 do
  def parse_input() do
    File.read!("inputs/day_23.input")
    |> String.split("\n")
    |> Enum.with_index(0)
    |> Enum.flat_map(fn {row, y} ->
      String.graphemes(row)
      |> Enum.with_index
      |> Enum.map(fn {el, x} -> {el, {y,x}} end)
    end)
    |> Enum.into(%{}, fn {el, pos} -> {pos,el} end)
  end

  def print_mat(elements, {size_y, size_x}) do
    Enum.each(Range.new(0, size_y-1), fn y ->
      Enum.each(Range.new(0, size_x-1), fn x ->
        IO.write(Map.get(elements, {y,x}))
      end)
      IO.puts("")
    end)
  end

  def get_neighboors({y,x}, graph, without_slope) do
    el = Map.get(graph, {y,x})
    cond do
      without_slope -> [{y+1,x}, {y-1,x}, {y, x+1}, {y, x-1}]
      el == "." -> [{y+1,x}, {y-1,x}, {y, x+1}, {y, x-1}]
      el == ">" -> [{y, x+1}]
      el == "<" -> [{y, x-1}]
      el == "^" -> [{y-1,x}]
      el == "v" -> [{y+1,x}]
      true -> []
    end
    |> Enum.reject(fn pos -> Map.get(graph, pos, "#") == "#" end)
  end


  def max_path(pos, end_pos, graph, without_slope, visited, cache_key) do
    {ey, ex} = end_pos
    {py, _} = pos
    # max possible distance between pos and end (Aprox)
    max_dist = (ey - py) * (ex + 1)

    if max_dist + Enum.count(visited) < Process.get(cache_key) do
      -9999999999999
    else
      neighboors_calc = get_neighboors(pos, graph, without_slope)
      |> Enum.reject(fn neighboor -> neighboor in visited end)
      |> Enum.map(fn neighboor ->
        max_path(neighboor, end_pos, graph, without_slope, MapSet.put(visited, pos), cache_key)
      end)

      children_count = cond do
        Enum.empty?(neighboors_calc) && pos == end_pos -> 0
        Enum.empty?(neighboors_calc) && pos != end_pos -> -9999999999999
        true -> Enum.max(neighboors_calc)
      end

      Process.put(cache_key, max(Process.get(cache_key), children_count + 1))
      children_count + 1
    end
  end

  def first_star() do
    cache_key = "first_#{:rand.uniform(1_000_000)}"
    Process.put(cache_key, 0)

    graph = parse_input()
    size_y = Map.to_list(graph) |> Enum.map(fn {{y,_}, _} -> y end) |> Enum.max()
    size_x = Map.to_list(graph) |> Enum.map(fn {{_,x}, _} -> x end) |> Enum.max()

    initial_pos = {0, 1}
    end_pos = {size_y, size_x - 1}

    max_path(initial_pos, end_pos, graph, false, MapSet.new(), cache_key) - 1
  end

  def second_star() do
    cache_key = "second_#{:rand.uniform(1_000_000)}"
    Process.put(cache_key, 0)

    graph = parse_input()
    size_y = Map.to_list(graph) |> Enum.map(fn {{y,_}, _} -> y end) |> Enum.max()
    size_x = Map.to_list(graph) |> Enum.map(fn {{_,x}, _} -> x end) |> Enum.max()

    initial_pos = {0, 1}
    end_pos = {size_y, size_x - 1}

    max_path(initial_pos, end_pos, graph, true, MapSet.new(), cache_key) - 1
  end
end
