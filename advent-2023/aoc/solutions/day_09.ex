defmodule DAY09 do
  def parse_input(problem) do
    input = File.read!("inputs/day_09.input")
    |> String.split("\n")

    case problem do
      1 -> input |> Enum.map(fn l ->
        String.split(l," ")
        |> Enum.map(&String.to_integer/1)
        |> Enum.reverse
      end)
      2 -> input |> Enum.map(fn l ->
        String.split(l," ")
        |> Enum.map(&String.to_integer/1)
      end)
    end
  end


  def reduce_seq(sequences) do
    seq = hd(sequences)
    cond do
      Enum.filter(seq, fn el -> el != 0 end) == [] -> sequences
      true -> reduce_seq([Enum.chunk_every(seq, 2,1, :discard)
              |> Enum.map(fn [a,b] -> a-b end) | sequences])
    end
  end

  def get_root_addition(seq) do
    reduce_seq([seq])
    |> Enum.map(fn l ->
      cond do
        l == [] -> 0
        true -> hd(l)
      end
    end)
    |> Enum.sum()
  end

  def first_star() do
    parse_input(1)
    |> Enum.map(&get_root_addition/1)
    |> Enum.sum()
  end

  def second_star() do
    parse_input(2)
    |> Enum.map(&get_root_addition/1)
    |> Enum.sum()
  end
end

# 1789635132
