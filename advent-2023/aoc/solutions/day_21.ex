defmodule DAY21 do
  def parse_input() do
    mat = File.read!("inputs/day_21.input")
    |> String.split("\n")
    |> Enum.with_index
    |> Enum.flat_map(fn {row, y} ->
      String.graphemes(row)
      |> Enum.with_index
      |> Enum.map(fn {el, x} ->
        {el, {y,x}}
      end)
    end)

    initial_pos = Enum.reduce(mat, nil, fn {el, pos}, acc ->
      cond do
        acc == nil && el == "S" -> pos
        true -> acc
      end
    end)

    {
      Enum.into(mat, %{}, fn {el, pos} -> {pos, el} end),
      {
        (mat |> Enum.map(fn {_, {_,y}} -> y end) |> Enum.max) + 1,
        (mat |> Enum.map(fn {_, {x,_}} -> x end) |> Enum.max) + 1,
      },
      initial_pos,
    }
  end

  def map_get_infinite({y,x}, {size_y, size_x}) do
    new_y = cond do
      y < 0 -> if (rem(y, size_y) == 0), do: 0, else: (rem(y, size_y) + size_y)
      y > 0 -> rem(y, size_y)
      true -> 0
    end

    new_x = cond do
      x < 0 -> if (rem(x, size_x) == 0), do: 0, else: (rem(x, size_x) + size_x)
      x > 0 -> rem(x, size_x)
      true -> 0
    end

    {new_y, new_x}
  end

  def is_valid_neighboor(pos, mat, size, is_infinite) do
    pos = if (is_infinite), do: map_get_infinite(pos, size), else: pos

    case Map.get(mat, pos) do
      nil -> false
      "#" -> false
      _ -> true
    end
  end


  def get_neighboors({y,x}, mat, size, is_infinite) do
    Enum.filter([
      {y-1,x},
      {y+1,x},
      {y,x+1},
      {y,x-1},
    ], fn pos -> is_valid_neighboor(pos, mat, size, is_infinite) end)
  end

  def simulate(_, _, _, current, 0), do: current
  def simulate(mat, size, is_infinite, current, step) do
    current_list = Enum.flat_map(current, fn node ->
      neighboors = get_neighboors(node, mat, size, is_infinite)
      |> Enum.reject(fn n -> n in current end)
      if Enum.count(neighboors) == 0, do: [node], else: neighboors
    end)

    current = MapSet.new(current_list)
    simulate(mat, size, is_infinite, current, step - 1)
  end

  def first_star() do
    {mat, size, initial_pos} = parse_input()

    steps = 2
    simulate(mat, size, false, MapSet.new([initial_pos]), steps) |> Enum.count
  end

  def second_star() do
    {mat, size, initial_pos} = parse_input()


    # generate 500 points
    steps = 500
    {_, seq} = Enum.reduce((1..steps), {MapSet.new([initial_pos]), []}, fn idx, {acc, acc_seq} ->
      IO.puts("Running #{idx}")
      acc = simulate(mat, size, true, acc, 1)

      {acc, acc_seq ++  [acc |> Enum.count]}
    end)

    # This will generate the first points that will be used to compute the solution after the interpolation
    IO.inspect(seq, limit: :infinity)
    # https://colab.research.google.com/drive/1PIXR6z07r67S8guQt6TFr37ffyO2t3Ng#scrollTo=AT680AN7UKl2

  end

end
