defmodule DAY03 do
  def get_numbers(input_data) do
    input_data
    |> String.split("\n")
    |> Enum.map((fn l -> [Regex.scan(~r/(\d+)/, l), Regex.scan(~r/(\d+)/, l, return: :index)] end))
    |> Enum.map(&Enum.zip(&1))
    |> Enum.map(fn match -> Enum.map(match, fn {[n,_], [idx,_]} -> {String.to_integer(n),idx} end) end)
    |> Enum.with_index
    |> Enum.reject(fn {r,_} -> Enum.empty?(r) end)
    |> Enum.map(fn {m, y} ->
      Enum.map(m, fn {v, r} -> {{r, y}, v} end)
    end)
    |> Enum.map(fn row ->
      Enum.flat_map(row, fn {{{ri, re}, y}, v} ->
        Enum.map(ri..ri+re-1, fn x -> {{y,x}, {v,y,ri,re}} end)
      end)
    end)
    |> List.flatten
    # {{y, x} => {v, y, ri, re}},
    |> Enum.into(%{}, fn {k, v} -> {k, v} end)
  end

  def find_symbols(input_data, pattern) do
    input_data
    |> String.split("\n")
    |> Enum.map(fn l -> Regex.scan(pattern, l, return: :index) end)
    |> Enum.with_index
    |> Enum.reject(fn {v,_} -> Enum.empty?(v) end)
    |> Enum.map(fn {v,i} -> {Enum.map(v, &List.first(&1)), i} end)
    |> Enum.map(fn {v,i} -> {Enum.map(v, fn {x,_} -> x end), i} end)
    |> Enum.flat_map(fn
      {x, y} -> Enum.map(x, fn l -> {y,l} end)
    end)
  end

  def get_neighboors({y,x}, l) do
    Enum.reject([
      {y-1,x-1},
      {y-1,x},
      {y-1,x+1},
      {y,x-1},
      {y,x+1},
      {y+1,x-1},
      {y+1,x},
      {y+1,x+1}
    ], fn {ny, nx} -> ny < 0 || nx < 0 || ny > l || nx > l end)
  end

  def first_star() do
    input_data = File.read!("inputs/day_03.input")
    l = String.length((List.first(String.split(input_data, "\n")))) - 1
    n_idxs = get_numbers(input_data)

    find_symbols(input_data, ~r/[^0-9.]/)
    |> Enum.map(fn p -> get_neighboors(p,l) end)
    |> List.flatten
    |> Enum.map(fn f ->
      Map.get(n_idxs, f, nil)
    end)
    |> Enum.uniq
    |> Enum.filter(&(&1 != nil))
    |> Enum.map(fn {v,_,_,_} -> v end)
    |> Enum.sum
  end

  def second_star() do
    input_data = File.read!("inputs/day_03.input")
    l = String.length((List.first(String.split(input_data, "\n")))) - 1
    n_idxs = get_numbers(input_data)

    find_symbols(input_data, ~r/\*/)
    |> Enum.map(fn p -> get_neighboors(p,l) end)
    |> Enum.map(fn s ->
      Enum.map(s, fn f ->
        Map.get(n_idxs, f, nil)
      end)
      |> Enum.uniq
      |> Enum.filter(&(&1 != nil))
    end )
    |> Enum.filter(fn s -> Enum.count(s) == 2 end)
    |> Enum.map(fn [{v1,_,_,_}, {v2,_,_,_}] -> v1 * v2 end)
    |> Enum.sum
  end

end
