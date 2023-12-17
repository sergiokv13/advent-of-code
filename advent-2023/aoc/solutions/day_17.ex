defmodule DAY17 do
  def parse_input() do
    list_mat = File.read!("inputs/day_17.input")
    |> String.split("\n")
    |> Enum.with_index
    |> Enum.map(fn {row, y} ->
      String.graphemes(row)
      |> Enum.with_index
      |> Enum.map(fn {el, x} ->
        {String.to_integer(el), {y,x}}
      end)
    end)

    size = {Enum.count(list_mat), Enum.count(hd(list_mat))}

    list_map = list_mat
    |> List.flatten
    |> Enum.reduce(%{}, fn {el, key}, acc ->
        Map.put(acc, key, el)
    end)

    {list_map, size}
  end

  def is_valid_neighboor({posy, posx}, {size_y, size_x}) do
    cond do
      posx < 0 || posy < 0 -> false
      posx >= size_x -> false
      posy >= size_y -> false
      true -> true
    end
  end

  # L-R-U-D
  def get_neighboors({{y,x}, {dir, dir_count}}, {size_y, size_x}, {min, max}) do
    cond do
      dir == "L" && dir_count >= min -> [{{y, x-1}, {"L", dir_count+1}}, {{y+1, x}, {"D",1}}, {{y-1, x}, {"U",1}}]
      dir == "L" -> [{{y, x-1}, {"L", dir_count+1}}]

      dir == "R" && dir_count >= min -> [{{y, x+1}, {"R", dir_count+1}}, {{y+1, x}, {"D",1}}, {{y-1, x}, {"U",1}}]
      dir == "R" -> [{{y, x+1}, {"R", dir_count+1}}]

      dir == "U" && dir_count >= min -> [{{y-1, x}, {"U", dir_count+1}}, {{y, x-1}, {"L",1}}, {{y, x+1}, {"R",1}}]
      dir == "U" -> [{{y-1, x}, {"U", dir_count+1}}]

      dir == "D" && dir_count >= min -> [{{y+1, x}, {"D", dir_count+1}}, {{y, x-1}, {"L",1}}, {{y, x+1}, {"R",1}}]
      dir == "D" -> [{{y+1, x}, {"D", dir_count+1}}]
    end
    |> Enum.filter(fn {pos,{_, dir_count}} ->
      is_valid_neighboor(pos, {size_y, size_x}) && dir_count <= max
    end)
  end

  def get_min_path(_,_, [], distances, _), do: distances
  def get_min_path(mat, size, queue, distances, {min, max}) do
    # This should be a priority queue to improve performance
    [curr_node | queue] = queue
    |> Enum.sort_by(fn n -> Map.get(distances, n) end)

    node_dist = Map.get(distances, curr_node)

    {new_distances, new_queue} = get_neighboors(curr_node, size, {min, max})
    |> Enum.reduce({distances, queue}, fn n, {acc_dist, acc_queue} ->

      curr_dist = Map.get(acc_dist, n, 9999999999)
      new_dist = node_dist + Map.get(mat, elem(n,0))

      if new_dist < curr_dist do
        { Map.put(acc_dist, n, new_dist), [n | acc_queue]}
      else
        { acc_dist, acc_queue }
      end
    end)

    get_min_path(mat, size, new_queue, new_distances, {min, max})
  end

  def first_star() do
    {mat, {size_y, size_x}} = parse_input()
    distances = get_min_path(
      mat,
      {size_y, size_x},
      [{{0,0}, {"R", 0}}, {{0,0}, {"D", 0}}],
      %{
        {{0,0}, {"R", 0}} => 0,
        {{0,0}, {"D", 0}} => 0,
      },
      {1, 3}
    )

    distances
      |> Map.to_list
      |> Enum.filter(fn {{{y,x}, _}, _} -> y == size_y-1 && x == size_x-1 end)
      |> Enum.min_by(fn {_, d} -> d end)

  end

  def second_star() do
    {mat, {size_y, size_x}} = parse_input()
    distances = get_min_path(
      mat,
      {size_y, size_x},
      [{{0,0}, {"R", 0}}, {{0,0}, {"D", 0}}],
      %{
        {{0,0}, {"R", 0}} => 0,
        {{0,0}, {"D", 0}} => 0,
      },
      {4, 10}
    )

    distances
      |> Map.to_list
      |> Enum.filter(fn {{{y,x}, _}, _} -> y == size_y-1 && x == size_x-1 end)
      |> Enum.filter(fn {{_, {_, dist_count}}, _} -> dist_count >= 4 end)
      |> Enum.min_by(fn {_, d} -> d end)
  end
end
