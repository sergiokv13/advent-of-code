defmodule DAY02 do

  def parse_game(game_str) do
    String.split(game_str, ":")
    |> List.last
    |> String.split(";")
    |> Enum.map(fn (round) ->
      round
      |> String.split(" ")
      |> Enum.map(&String.replace(&1, ",", ""))
      |> (fn ([_ | t]) -> t end).()
      |> Enum.chunk_every(2)
      |> Enum.reduce(%{}, fn [prev, color], acc -> Map.put(acc, color, String.to_integer(prev)) end)
    end)
  end

  def parse_input() do
    File.read!("inputs/day_02.input")
    |> String.split("\n")
    |> Enum.map(&parse_game/1)
  end

  def first_star() do
    limits = %{ "red" => 12, "green" => 13, "blue" => 14 }
    parse_input()
    |> Enum.with_index
    |> Enum.filter(fn {s, _} ->
      Enum.all?(s, fn g ->
        Enum.all?(g, fn {key, value} ->
            Map.get(limits, key) >= value
          end)
        end)
      end)
    |> Enum.map(fn {_, idx} -> idx + 1 end)
    |> Enum.sum()
  end

  def second_star() do
    # parse_input()
    base_map = %{ "red" => 0, "green" => 0, "blue" => 0 }

    parse_input()
    |> Enum.map(fn game ->
        game
        |> Enum.reduce(
        base_map,
        fn (g, acc) ->
          Enum.reduce(g, acc, fn {key, v}, iacc ->
            Map.put(iacc, key, max(v, Map.get(iacc, key)))
          end)
        end
      )
      |> Enum.reduce(1, fn {_, v}, acc -> v * acc end)
    end)
    |> Enum.sum

  end
end
