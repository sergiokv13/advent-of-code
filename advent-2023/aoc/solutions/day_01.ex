defmodule DAY01 do
  def first_star() do
    File.read!("inputs/day_02.input")
    |> String.split("\n")
    |> Enum.map(&String.replace(&1, ~r/\D/, ""))
    |> Enum.map(fn nums -> "#{String.first(nums)}#{String.last(nums)}" end)
    |> Enum.map(&String.to_integer/1)
    |> Enum.sum()
  end

  def replace_edges(value) do
    mapped = %{
      "one" => "1",
      "two" => "2",
      "three" => "3",
      "four" => "4",
      "five" => "5",
      "six" => "6",
      "seven" => "7",
      "eight" => "8",
      "nine" => "9",
    }

    min_max_idxs = Map.keys(mapped)
    |> Enum.map(fn el -> {el, String.split(value, el)} end)
    |> Enum.filter(fn {_,splitted} -> Enum.count(splitted) > 1 end)
    |> Enum.map(fn {key,splitted} -> {
        key,
        Enum.map(splitted, &String.length/1)
        |> (fn v -> [hd(v), List.last(v)] end).()
      } end)

    if !Enum.empty?(min_max_idxs) do
      {first_lit, _} = min_max_idxs |> Enum.min_by(fn {_, [min, _]} -> min end)
      {last_lit, _} = min_max_idxs |> Enum.min_by(fn {_, [_, max]} -> max end)

      value
      |> String.replace(first_lit, "#{mapped[first_lit]}#{first_lit}", global: false)
      |> String.replace(last_lit, mapped[last_lit])
    else
      value
    end
  end

  def second_star do
    File.read!("inputs/day_01.input")
    |> String.split("\n")
    |> Enum.map(fn n -> replace_edges(n) end)
    |> Enum.map(&String.replace(&1, ~r/\D/, ""))
    |> Enum.map(fn nums -> "#{String.first(nums)}#{String.last(nums)}" end)
    |> Enum.map(&String.to_integer/1)
    |> Enum.sum()
  end

end
