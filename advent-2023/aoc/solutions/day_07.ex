defmodule DAY07 do
  @high_card %{:"A" => 14, :"K" => 13, :"Q" => 12, :"J" => 11, :"T" => 10, :"9"=> 9, :"8" => 8, :"7" => 7, :"6" => 6, :"5" => 5, :"4" => 4, :"3" => 3, :"2" => 2}
  @high_card2 %{:"A" => 14, :"K" => 13, :"Q" => 12, :"T" => 10, :"9"=> 9, :"8" => 8, :"7" => 7, :"6" => 6, :"5" => 5, :"4" => 4, :"3" => 3, :"2" => 2, :"J" => 1}

  def parse_input() do
    File.read!("inputs/day_07.input")
    |> String.split("\n")
    |> Enum.map(fn row ->
      [cards, bet] = String.split(row, " ")
      {String.graphemes(cards), String.to_integer(bet)}
    end)
  end

  def get_best_with_wildcard(draw_map, number_of_js) do
    draw_map = Map.delete(draw_map, "J")

    {best_key, best_num} = Map.to_list(draw_map) |> Enum.max_by(fn {_, v} -> v end)
    {higher_ranked, _} = Map.to_list(draw_map) |> Enum.max_by(fn {k, _} -> Map.get(@high_card2,k) end)

    cond do
      best_num > 1 -> Map.put(draw_map, best_key, Map.get(draw_map, best_key) + number_of_js)
      true -> Map.put(draw_map, higher_ranked, Map.get(draw_map, higher_ranked) + number_of_js)
    end
  end

  @spec get_draw_value(map(), boolean()) :: 100 | 101 | 102 | 103 | 104 | 105 | 106
  def get_draw_value(draw_map, with_wild_card) do
    number_of_js = Map.get(draw_map, "J") || 0
    draw_map = cond do
      Kernel.not(with_wild_card) -> draw_map
      number_of_js == 0 -> draw_map
      number_of_js == 5 -> draw_map
      true -> get_best_with_wildcard(draw_map, number_of_js)
    end


    ocurrences = Map.values(draw_map)
    cond do
      Enum.any?(ocurrences, &(&1 >= 5)) -> 106
      Enum.any?(ocurrences, &(&1 >= 4)) -> 105
      Enum.any?(ocurrences, &(&1 == 3)) && Enum.any?(ocurrences, &(&1 == 2)) -> 104
      Enum.any?(ocurrences, &(&1 == 3)) -> 103
      Enum.count(Enum.filter(ocurrences, fn el -> el == 2 end)) == 2 -> 102
      Enum.any?(ocurrences, &(&1 == 2)) -> 101
      true -> 100
    end
  end

  def get_by_high_card(a, b, with_wild_card) do
    rank = if with_wild_card do @high_card2 else @high_card end
    Enum.zip(a,b) |> Enum.reduce(nil, fn {ael,bel}, acc ->
      cond do
        acc !== nil -> acc
        Map.get(rank, String.to_atom(ael)) > Map.get(rank, String.to_atom(bel)) -> true
        Map.get(rank, String.to_atom(ael)) < Map.get(rank, String.to_atom(bel)) -> false
        true -> nil
      end
     end)
  end

  def comparison(a , b, with_wild_card) do
    draw_value_a = a |> Enum.reduce(%{}, fn letter, acc ->
      case Map.get(acc, letter) do
        nil -> Map.put(acc, letter, 1)
        count -> Map.put(acc, letter, count + 1)
      end
    end)
    |> (fn el -> get_draw_value(el, with_wild_card) end).()

    draw_value_b = b |> Enum.reduce(%{}, fn letter, acc ->
      case Map.get(acc, letter) do
        nil -> Map.put(acc, letter, 1)
        count -> Map.put(acc, letter, count + 1)
      end
    end)
    |> (fn el -> get_draw_value(el, with_wild_card) end).()

    cond do
      draw_value_a > draw_value_b -> true
      draw_value_b > draw_value_a -> false
      true -> if (get_by_high_card(a,b, with_wild_card) !== nil) do get_by_high_card(a,b, with_wild_card) else true end
    end
  end

  def first_star() do
    Enum.sort(
      parse_input(),
      fn {a,_},{b,_} -> comparison(a,b, false) end
    )
    |> Enum.reverse
    |> Enum.with_index(1)
    |> Enum.reduce(0, fn {{_,el}, idx}, acc -> (el*idx) + acc end)
  end

  def second_star() do
    Enum.sort(
      parse_input(),
      fn {a,_},{b,_} -> comparison(a,b, true) end
    )
    |> Enum.reverse
    |> Enum.with_index(1)
    |> Enum.reduce(0, fn {{_,el}, idx}, acc -> (el*idx) + acc end)
  end
end
