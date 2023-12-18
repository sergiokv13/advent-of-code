defmodule DAY18_DFS do
  def get_instructions() do
    File.read!("inputs/day_18.input")
    |> String.split("\n")
    |> Enum.map(&String.split/1)
    |> Enum.map(fn [dir, v, code] ->
      {
        dir,
        String.to_integer(v),
        code
          |> String.replace(")", "")
          |> String.replace("(", "")
          |> String.replace("#", "")
      }
    end)
  end

  def build_borders([], _, _), do: %{}
  def build_borders(instructions, {y,x}, idx) do
    [{dir, v, c} | rest] = instructions

    {new_map, last_pos} = case dir do
      "R" -> x..(x+v) |> Enum.reduce({%{},{}}, fn nx, {acc,_} -> { Map.put(acc, {y,nx}, c), {y,nx}} end)
      "L" -> x..(x-v) |> Enum.reduce({%{},{}}, fn nx, {acc,_} -> { Map.put(acc, {y,nx}, c), {y,nx}} end)
      "D" -> y..(y+v) |> Enum.reduce({%{},{}}, fn ny, {acc,_} -> { Map.put(acc, {ny,x}, c), {ny,x}} end)
      "U" -> y..(y-v) |> Enum.reduce({%{},{}}, fn ny, {acc,_} -> { Map.put(acc, {ny,x}, c), {ny,x}} end)
    end

    Map.merge(new_map, build_borders(rest, last_pos, idx + 1))
  end

  def count_inner(mat, {y,x}, {{min_y, max_y}, {min_x, max_x}}, cache_key) do
    if (Process.get({cache_key, {y,x}}, false)) do {0, true}
    else
      Process.put({cache_key, {y,x}}, true)

      {counter, valid, finished} = cond do
        y < min_y || x < min_x -> {0, false, true}
        y > max_y || x > max_x -> {0, false, true}
        Map.get(mat, {y,x}, ".") != "." -> {0, true, true}
        Map.get(mat, {y,x}, ".") == "." -> {1, true, false}
      end

      cond do
        !valid -> {0, false}
        finished -> {0, true}
        true -> (fn ->
          {cd, vd} = count_inner(mat, {y+1,x}, {{min_y, max_y}, {min_x, max_x}}, cache_key)
          {cu, vu} = count_inner(mat, {y-1,x}, {{min_y, max_y}, {min_x, max_x}}, cache_key)
          {cr, vr} = count_inner(mat, {y,x+1}, {{min_y, max_y}, {min_x, max_x}}, cache_key)
          {cl, vl} = count_inner(mat, {y,x-1}, {{min_y, max_y}, {min_x, max_x}}, cache_key)

          if (vd && vu && vr && vl),
            do: {counter + cd + cu + cr + cl, true},
            else: {0, false}
        end).()
      end
    end
  end

  def first_star() do
    cache_key = "first_#{:rand.uniform(1_000_000)}"
    instructions = get_instructions()

    mat = build_borders(instructions, {0,0}, 1)

    max_y = (mat |> Map.keys |> Enum.max_by(fn {y,_} -> y end) |> (fn {y,_} -> y end).())
    min_y = (mat |> Map.keys |> Enum.min_by(fn {y,_} -> y end) |> (fn {y,_} -> y end).())

    max_x = (mat |> Map.keys |> Enum.max_by(fn {_,x} -> x end) |> (fn {_,x} -> x end).())
    min_x = (mat |> Map.keys |> Enum.min_by(fn {_,x} -> x end) |> (fn {_,x} -> x end).())

    initial_size = Map.to_list(mat) |> Enum.count

    positions = for y <- (min_y..max_y), x <- (min_x..max_x), do: {y, x}
    reduced = Enum.reduce(positions, 0, fn pos, acc ->
      if (acc != 0) do
        acc
      else
        {c, v} = count_inner(mat, pos, {{min_y, max_y}, {min_x, max_x}}, cache_key)
        if (v), do: c, else: acc
      end
    end)

    reduced + initial_size
  end
end
