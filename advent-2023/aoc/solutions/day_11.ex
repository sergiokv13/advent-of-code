defmodule DAY11 do
  def get_empty() do
    parsed = File.read!("inputs/day_11.input")
    |> String.split("\n")
    |> Enum.map(&String.graphemes/1)

    empty_rows = Enum.with_index(parsed)
                |> Enum.filter(fn {row,_} ->
                  Enum.all?(row, fn el -> el == "." end)
                end)
                |> Enum.map(fn {_, idx} -> idx end)

    empty_cols = 0..(Enum.count(hd(parsed))-1)
                |> Enum.map(fn idx ->
                  Enum.map(parsed, fn row -> Enum.at(row, idx) end)
                end)
                |> Enum.with_index
                |> Enum.filter(fn {row,_} ->
                  Enum.all?(row, fn el -> el == "." end)
                end)
                |> Enum.map(fn {_, idx} -> idx end)

    { empty_rows, empty_cols}
  end

  def get_galaxies() do
    File.read!("inputs/day_11.input")
    |> String.split("\n")
    |> Enum.with_index
    |> Enum.flat_map(fn {row, y} ->
      String.graphemes(row)
        |> Enum.with_index
        |> Enum.map(fn {el,x} -> {{y, x}, el} end)
      end
    ) |> Enum.filter(fn {_, el} -> el == "#" end)
    |> Enum.map(fn {pos, _} -> pos end)
  end

  def get_steps({gy1, gx1}, {gy2, gx2}, { empty_rows, empty_cols }, time_steps) do
    n_x = empty_cols
      |> Enum.filter(fn el -> el in gx1..gx2 || el in gx2..gx1 end)
      |> Enum.count
    n_x = time_steps*n_x

    n_y = empty_rows
      |> Enum.filter(fn el -> el in gy1..gy2 || el in gy2..gy1 end)
      |> Enum.count
    n_y = time_steps*n_y

    d_x = abs(gx1 - gx2) + n_x
    {d_x,r_x} = if (rem(d_x,2) == 0), do: {d_x,0}, else: {d_x-1,1}

    d_y = abs(gy1 - gy2) + n_y
    {d_y,r_y} = if (rem(d_y,2) == 0), do: {d_y,0}, else: {d_y-1,1}

    dist = d_x+d_y+r_x+r_y
    dist
  end

  def first_star() do
    empty = get_empty()
    galaxies = get_galaxies()

    all_pairs = galaxies |> Enum.map(fn g1 ->
      galaxies |> Enum.filter(fn g2 -> g2 !== g1 end)
               |> Enum.map(fn g2 -> get_steps(g1,g2,empty,1) end)
               |> Enum.sum()
    end) |> Enum.sum()
    all_pairs / 2
  end

  def second_star() do
    empty = get_empty()
    galaxies = get_galaxies()

    all_pairs = galaxies |> Enum.map(fn g1 ->
      galaxies |> Enum.filter(fn g2 -> g2 !== g1 end)
               |> Enum.map(fn g2 -> get_steps(g1,g2,empty,1000000-1) end)
               |> Enum.sum()
    end) |> Enum.sum()
    all_pairs / 2
  end
end
