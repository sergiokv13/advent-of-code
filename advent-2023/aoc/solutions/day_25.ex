defmodule DAY25 do
  def parse_input() do
    File.read!("inputs/day_25.input")
    |> String.split("\n")
    |> Enum.reduce(%{}, fn row, acc ->
      [parent, children] = String.split(row, ": ")
      children = String.split(children, " ")
      acc = Map.put(acc, parent, Map.get(acc, parent, []) ++ children)

      Enum.reduce(children, acc, fn c, a ->
        Map.put(a, c, Map.get(acc, c, []) ++ [parent])
      end)
    end)
    |> Map.to_list
    |> Enum.into(%{}, fn {key, val} -> {key, MapSet.new(val)} end)
  end


  def get_longest_path([], _, _ , _, parent, nil), do: parent
  def get_longest_path([], _, _ , _, parent, ln) do
    Enum.reduce_while(parent, [ln], fn _, acc ->
      current = hd(acc)
      curr_p = Map.get(parent, current, nil)
      case curr_p != nil do
        true ->
          {:cont, [curr_p] ++ acc}
        false ->
          {:halt, acc}
      end
    end)
    |> Enum.chunk_every(2, 1)
    |> Enum.reject(fn el -> Enum.count(el) < 2 end)
  end

  def get_longest_path(queue, graph, skip, visited, parent, ln) do
    [current | rest] = queue
    visited = MapSet.put(visited, current)
    neighboors = (Map.get(graph, current)
    |> Enum.reject(fn n -> n in visited end))
    |> Enum.reject(fn n ->
      [current, n] in skip || [n, current] in skip
    end)

    queue = rest ++ neighboors

    parent = Enum.reduce(neighboors, parent, fn n, acc ->
      Map.put(acc, n, current)
    end)

    get_longest_path(
      queue,
      graph,
      skip,
      visited,
      parent,
      ln
    )
  end


  def filter_connection(connections, _, _, _, _, 0), do: connections
  def filter_connection(connections, graph, root, ln, skip, lvl) do
    Enum.map(connections, fn conn ->
      skip = MapSet.put(skip, conn)

      connections = get_longest_path([root], graph, skip, MapSet.new(), %{}, ln)

      if (Enum.empty?(connections)), do: Process.put("skip", skip)

      filter_connection(connections, graph, root, ln, skip, lvl - 1)
    end)
  end

  def get_random_elements(list, n) when n <= length(list) do
    list |> Enum.uniq() |> Enum.shuffle() |> Enum.take(n)
  end

  def first_star() do
    graph = parse_input()
    Process.put("skip", nil)

    all_nodes = graph |> Map.to_list |> Enum.map(fn {p,c} -> [p] ++ MapSet.to_list(c) end)
      |> List.flatten
      |> MapSet.new
      |> Enum.to_list

    Enum.reduce_while(all_nodes, [], fn _,_ ->
      skip = Process.get("skip", nil)
      case skip == nil do
        true ->
          [root, ln] = get_random_elements(all_nodes, 2)
          # most of the selected points will be on different parts of the graph
          # so this should try just a couple of times

          connections = get_longest_path(
            [root],
            graph,
            MapSet.new([]),
            MapSet.new(),
            %{},
            ln
          )

          filter_connection(connections, graph, root, ln, MapSet.new([]), 3)
          {:cont, false}
        false ->
          {:halt, true}
      end
    end)

    skip = Process.get("skip")

    [root, _] = get_random_elements(all_nodes, 2)
    # Now let's count the cycles
    first_cycle_len = get_longest_path(
      [root],
      graph,
      skip,
      MapSet.new(),
      %{},
      nil
    )
    |> Map.to_list
    |> Enum.map(fn {p,c} -> [p,c] end)
    |> List.flatten
    |> MapSet.new
    |> Enum.count

    total_len = all_nodes |> Enum.count

    first_cycle_len * (total_len - first_cycle_len)
  end
end
