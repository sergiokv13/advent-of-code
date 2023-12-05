defmodule DAY04 do
  def get_numbers(input_data) do
    input_data
    |> String.split("\n")
    |> Enum.map(fn l -> List.last(String.split(l, ":")) end)
    |> Enum.map(fn l ->
      String.split(l, "|")
      |> Enum.map(fn i ->
        String.split(i, " ")
        |> Enum.filter(fn ie -> String.length(ie) > 0 end)
      end)
      |> (fn section ->
        {
          hd(section),
          List.last(section)
          |> MapSet.new()
        }
      end).()
    end)
  end

  def get_card_points() do
    input_data = File.read!("inputs/day_04.input")
    get_numbers(input_data)
    # {numbers, winning_numbers_set}[]
    |> Enum.map(fn {ns, wn} ->
      Enum.filter(ns, fn n -> MapSet.member?(wn, n) end)
    end)
    |> Enum.map(&Enum.count/1)
  end

  def first_star() do
    get_card_points()
    |> Enum.filter(fn c -> c !== 0 end)
    |> Enum.map(fn c -> :math.pow(2,c-1) end)
    |> Enum.sum()
  end

  def get_cards(_,cards,[]), do: cards
  def get_cards(points_by_card, cards, todo) do
    {cidx, todo} = List.pop_at(todo, 0)
    cards = [(cidx) | cards]
    card_p = Map.get(points_by_card, cidx)
    todo = if card_p >= 1, do: Enum.reduce(1..card_p, todo, fn idx, acc -> [(idx + cidx) | acc] end), else: todo

    get_cards( points_by_card, cards, todo)
  end


  def second_star() do
    idx_points = get_card_points() |> Enum.with_index(1)
    points_by_card = idx_points
      |> Enum.into(%{}, fn {k, v} -> {v,k} end )

    get_cards(points_by_card, [], idx_points |> Enum.map(fn {_,idx} -> idx end))
    |> Enum.count
  end
end
