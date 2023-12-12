defmodule DAY12 do
  def parse_input() do
    File.read!("inputs/day_12.input")
    |> String.split("\n")
    |> Enum.map(fn row ->
      [hs, ns] = String.split(row)
      [
        String.graphemes(hs),
        String.split(ns, ",") |> Enum.map(&String.to_integer/1)
      ]
    end)
  end

  def count_seq_of([], _), do: 0
  def count_seq_of(hs, el) do
    case hd(hs) do
      ^el -> 1 + count_seq_of(tl(hs), el)
      # wildcard
      "?" -> 1 + count_seq_of(tl(hs), el)
      _ -> 0
    end
  end

  def get_memo_key(hs, ns) do
    {hs, ns}
  end

  def count_possible([], ns, memo), do: (if Enum.empty?(ns), do: {1, memo}, else: {0, memo})
  def count_possible(hs, [], memo), do: (if ("#" in hs), do: {0, memo}, else: {1, memo})
  def count_possible(hs, ns, memo) do
    memo_key = get_memo_key(hs, ns)
    memoized = Map.get(memo, memo_key)
    if (memoized != nil) do
      {memoized, memo}
    else
      resp = case hd(hs) do
        "." -> count_possible(tl(hs), ns, memo)
        "#" -> (fn ->
          if (hd(ns) <= count_seq_of(hs, "#")),
          do: (fn ->
                new_hs = Enum.drop(hs, hd(ns))
                cond do
                  new_hs != [] && hd(new_hs) == "#" -> {0, memo}
                  new_hs != [] && hd(new_hs) == "?" -> count_possible(["."] ++ tl(new_hs), tl(ns), memo)
                  true -> count_possible(new_hs, tl(ns), memo)
                end
              end).(),
          else: {0,memo}
        end).()
        "?" -> (fn ->
          {dot_c, dot_m} = count_possible(["."] ++ tl(hs), ns, memo)
          {hash_c, hash_m} = count_possible(["#"] ++ tl(hs), ns, Map.merge(dot_m, memo))
          { dot_c + hash_c, hash_m}
        end).()
      end

      # Memo
      {resp_v, resp_m} = resp
      resp_m = Map.put(resp_m, memo_key, resp_v)
      {resp_v, resp_m}
    end
  end

  def first_star() do
      parse_input()
      |> Enum.reduce(0, fn [hs,ns], acc ->
          acc +  elem(count_possible(hs, ns, %{}), 0)
      end)
  end

  def second_star() do
    parse_input()
    |> Enum.map(fn [hs, ns] ->
        [
          hs ++ ["?"] ++ hs ++ ["?"] ++ hs ++ ["?"] ++ hs ++ ["?"] ++ hs,
          ns ++ ns ++ ns ++ ns ++ ns
        ]
      end)
      |> Enum.reduce(0, fn [hs,ns], acc ->
        acc +  elem(count_possible(hs, ns, %{}), 0)
      end)
  end
end
