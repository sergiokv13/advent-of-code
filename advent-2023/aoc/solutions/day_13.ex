defmodule DAY13 do
  def parse_pattern(raw_pattern) do
    for_rows = raw_pattern |> String.split("\n")
    mat = raw_pattern |> String.split("\n") |> Enum.map(&String.graphemes/1)

    for_cols = 0..(Enum.count(hd(mat))-1)
    |> Enum.map(fn idx ->
      Enum.map(mat, fn row -> Enum.at(row, idx) end)
      |> Enum.reverse |> Enum.join("")
    end)

    { for_rows, for_cols }
  end


  def check_mirror(strings, size, left, right) do
    cond do
      left < 0 -> true
      right >= size -> true
      true -> (fn ->
        Enum.at(strings, left) == Enum.at(strings, right)
          && check_mirror(strings, size, left - 1, right + 1)
      end).()
    end
  end

  def find_mirror_idx(strings, skip_idx \\ -1) do
    size = Enum.count(strings)
    Enum.reduce(0..size-2, -1, fn idx, acc ->
      valid = check_mirror(strings, size, idx, idx+1)
      cond do
        valid && idx != skip_idx -> idx
        true -> acc
      end
    end)
  end

  def parse_input() do
    File.read!("inputs/day_13.input")
    |> String.split("\n\n")
    |> Enum.map(&parse_pattern/1)
  end

  def summarize({for_rows, for_cols}) do
    for_cols = find_mirror_idx(for_cols)
    for_rows = find_mirror_idx(for_rows)

    cond do
      for_cols != -1 -> {(for_cols + 1), {for_cols, "cols"}}
      for_rows != -1 -> {(for_rows + 1) * 100, {for_rows, "rows"}}
      true -> 0
    end
  end

  def get_all_options_for_str(string) do
    str_chars = String.graphemes(string)

    str_chars
    |> Enum.with_index
    |> Enum.map(fn {_, old_el_idx} ->
      Enum.map(Enum.with_index(str_chars), fn {el, el_idx} ->
        cond do
          old_el_idx == el_idx && el == "." -> "#"
          old_el_idx == el_idx && el == "#" -> "."
          true -> el
        end
      end) |> Enum.join("")
    end)
  end

  def get_all_options(strings) do
    Enum.with_index(strings) |> Enum.flat_map(fn {string, idx} ->
      new_strings = get_all_options_for_str(string)

      new_strings |> Enum.map(fn new_string ->
        Enum.map(Enum.with_index(strings), fn {old_string, old_idx} ->
          if (old_idx == idx), do: new_string, else: old_string
         end)
      end)

    end)
  end

  def summarize_with_smudge_fix({{for_rows, for_cols}, {match_idx, match_type}}) do
    for_cols = get_all_options(for_cols)
    |> Enum.reduce(-1, fn strings, acc ->
      if acc != -1 do acc
      else
        find_mirror_idx(strings, if (match_type == "cols") do match_idx else -1 end)
      end
    end)

    if (for_cols != -1) do
      for_cols + 1
    else
      for_rows = get_all_options(for_rows)
      |> Enum.reduce(-1, fn strings, acc ->
        if acc != -1 do acc
        else
          find_mirror_idx(strings, if (match_type == "rows") do match_idx else -1 end)
        end
      end)
      (for_rows + 1) * 100
    end
  end

  def first_star() do
    parse_input()
      |> Enum.map(&summarize/1)
      |> Enum.map(fn {v,_} -> v end)
      |> Enum.sum()
  end

  def second_star() do
    matches = parse_input()
    |> Enum.map(&summarize/1)
    |> Enum.map(fn {_,m} -> m end)

    parse_input()
      |> Enum.zip(matches)
      |> Enum.map(&summarize_with_smudge_fix/1)
      |> Enum.sum()
  end
end
