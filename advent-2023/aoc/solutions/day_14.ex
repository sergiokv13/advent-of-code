defmodule DAY14 do
  def parse_input() do
    mat = File.read!("inputs/day_14.input")
    |> String.split("\n")
    |> Enum.map(&String.graphemes/1)

    0..(Enum.count(mat)-1)
    |> Enum.map(fn idx ->
      Enum.reverse(Enum.map(mat, fn row -> Enum.at(row, idx) end))
    end)
  end

  def collapse([]), do: []
  def collapse(els) do
    cached = Process.get({"collapse", els})
    if cached do
      cached
    else
      head = hd(els)
      next_seq = collapse(tl(els))

      next = if next_seq == [], do: nil, else: hd(next_seq)
      rest = if next, do: tl(next_seq), else: []

      res = cond do
        head == "O" && next == "." -> [next] ++ collapse([head] ++ rest)
        next == nil -> [head]
        true -> [head] ++ [next] ++ rest
      end
      Process.put({"collapse", els}, res)
      res
    end
  end

  def count_row(els) do
    els
      |> Enum.with_index(1)
      |> Enum.filter(fn {el, _} -> el == "O" end)
      |> Enum.map(fn {_, idx} -> idx end)
      |> Enum.sum
  end

  def rotate(mat) do
    cached = Process.get({"rotate", mat})
    if cached do cached
    else
      rotated = 0..(Enum.count(mat)-1)
      |> Enum.map(fn idx ->
        Enum.reverse(Enum.map(mat, fn row -> Enum.at(row, idx) end))
      end)
      Process.put({"rotate", mat}, rotated)
      rotated
    end
  end

  def first_star() do
    parse_input()
    |> Enum.map(&collapse/1)
    |> Enum.map(&count_row/1)
    |> Enum.sum
  end

  def second_star() do
  mat =  parse_input()

  # iterations = 1000000000
  consolidation_point = 10000 # magic number (10k, 100k)

  {_, seq} = (1..consolidation_point) |> Enum.reduce({mat, []}, fn _, {acc_mat, seq} ->
    new_mat = (1..4) |> Enum.reduce(acc_mat, fn _, acc_i ->
      new_mat = collapse(
        acc_i |> Enum.map(&collapse/1)
      )
      rotate(new_mat)
    end)
    {new_mat, [new_mat |> Enum.map(&count_row/1) |> Enum.sum | seq]}
  end)
  Enum.at(seq, 0)
  end
end
