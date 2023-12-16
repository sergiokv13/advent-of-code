defmodule DAY16 do
  def parse_input() do
    list_mat = File.read!("inputs/day_16.input")
    |> String.split("\n")
    |> Enum.with_index
    |> Enum.map(fn {row, y} ->
      String.graphemes(row)
      |> Enum.with_index
      |> Enum.map(fn {el, x} ->
        {el, {y,x}}
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

  def is_valid_neighboor(pos, {size_y, size_x}) do
    case pos do
      {-1, _} -> false
      {_, -1} -> false
      {_, ^size_x} -> false
      {^size_y, _} -> false
      _ -> true
    end
  end

  def get_neighboors({y,x}, el, {dir_y, dir_x}, {size_y, size_x}) do
    cond do
      el == "." -> [{{y + dir_y, x + dir_x}, {dir_y, dir_x}}]
      el == "-" && dir_x != 0 -> get_neighboors({y,x}, ".", {dir_y, dir_x}, {size_y, size_x})
      el == "|" && dir_y != 0 -> get_neighboors({y,x}, ".", {dir_y, dir_x}, {size_y, size_x})
      el == "-" -> [{{y, x + 1}, {0,1}}, {{y, x - 1}, {0,-1}}]
      el == "|" -> [{{y + 1, x}, {1,0}}, {{y - 1, x}, {-1,0}}]

      el == "\\" && dir_x > 0-> [{{y + 1, x}, {1,0}}]
      el == "\\" && dir_x < 0-> [{{y - 1, x}, {-1,0}}]
      el == "\\" && dir_y > 0-> [{{y, x + 1}, {0,1}}]
      el == "\\" && dir_y < 0-> [{{y, x - 1}, {0,-1}}]

      el == "/" && dir_x > 0-> [{{y - 1, x}, {-1,0}}]
      el == "/" && dir_x < 0-> [{{y + 1, x}, {1,0}}]
      el == "/" && dir_y > 0-> [{{y, x - 1}, {0,-1}}]
      el == "/" && dir_y < 0-> [{{y, x + 1}, {0,1}}]
    end
    |>
    Enum.filter(fn {pos,_}->
      is_valid_neighboor(pos, {size_y, size_x})
    end)
  end

  def get_starting_points({size_x, size_y}) do
    Enum.flat_map((0..size_y - 1), fn y ->
      if y != 0 || y != size_y - 1 do
        Enum.flat_map((0..size_x - 1), fn x ->
          if x != 0 || x != size_x - 1 do
            Enum.map([{0,1}, {0,-1}, {1,0}, {-1,0}], fn pos ->
              {{y,x}, pos}
            end)
          else [] end
        end)
      else [] end
    end)
    |> Enum.filter(fn {{y,x}, _} ->
        y == 0 || x == 0 || x == size_x - 1 || y == size_y - 1
    end)
  end

  def simulate({pos, dir}, mat, size, cache_key) do
    Process.put({ cache_key, pos, dir }, true)
    get_neighboors(pos, Map.get(mat, pos), dir, size)
    |> Enum.reject(fn {pos, dir} -> Process.get({ cache_key, pos, dir }) != nil end)
    |> Enum.each(fn c -> simulate(c, mat, size, cache_key) end)
  end

  def first_star() do
    cache_key = "first_#{:rand.uniform(1_000_000)}"
    {mat, size} = parse_input()
    simulate({{0,0},{0,1}}, mat, size, cache_key)

    Process.get_keys()
    |> Enum.filter( fn key -> is_tuple(key) && elem(key, 0) == cache_key end)
    |> Enum.map(fn {_, pos, _} -> pos end)
    |> Enum.uniq
    |> Enum.count
  end

  def second_star() do
    {mat, {size_y, size_x}} = parse_input()

    IO.puts("Expected: #{Enum.count(get_starting_points({size_y, size_x}))}")

    get_starting_points({size_y, size_x})
    |> Enum.with_index
    |> Enum.map(fn {{pos, dir}, idx} ->
      if (rem(idx, 100) == 0), do: IO.puts(idx)

        cache_key = "second_#{:rand.uniform(1_000_000)}"
        simulate({pos, dir}, mat, {size_y, size_x}, cache_key)

        Process.get_keys()
        |> Enum.filter( fn key -> is_tuple(key) && elem(key, 0) == cache_key end)
        |> Enum.map(fn {_, pos, _} -> pos end)
        |> Enum.uniq
        |> Enum.count
    end)
    |> Enum.max()
  end
end
