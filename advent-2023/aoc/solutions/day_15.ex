defmodule DAY15 do
  def parse_input() do
    File.read!("inputs/day_15.input")
    |> String.split(",")
  end

  def get_hash(string) do
    String.codepoints(string)
    |> Enum.map(fn <<v::utf8>> -> v end)
    |> Enum.reduce(0, fn el, acc ->
      rem((acc + el) * 17, 256)
    end)
  end

  def perform_instruction(string, hmap) do
    cond do
      String.contains?(string, "-") -> (fn ->
        label = hd(String.split(string, "-"))
        hashed_label = get_hash(label)
        in_box = Map.get(hmap, hashed_label, [])

        Map.put(
          hmap,
          hashed_label,
          in_box |> Enum.filter(fn {l, _} -> l != label end)
        )
      end).()
      String.contains?(string , "=") -> (fn ->
        label = hd(String.split(string, "="))
        hashed_label = get_hash(label)
        lens = String.to_integer(List.last(String.split(string, "=")))
        in_box = Map.get(hmap, hashed_label, [])

        already_added = !!Enum.find(in_box, fn {l, _} -> l == label end)
        new_box = case already_added do
          true -> in_box
            |> Enum.map(fn {l, le} -> if (l == label), do: {label,lens}, else: {l,le} end)
          false -> [{label, lens} |in_box]
        end

        Map.put(hmap, hashed_label, new_box)
      end).()
    end
  end

  def get_box_value({ box_id, box}) do
    Enum.reverse(box)
    |> Enum.with_index(1)
    |> Enum.map(fn {{_, v}, idx} -> v * idx * (box_id + 1) end)
    |> Enum.sum()
  end

  def first_star() do
    parse_input()
    |> Enum.map(fn el -> get_hash(el) end)
    |> Enum.sum()
  end

  def second_star() do
    parse_input()
    |> Enum.reduce(%{}, fn el, acc ->
      perform_instruction(el, acc)
    end)
    |> Map.to_list
    |> Enum.filter(fn {_, box} -> !Enum.empty?(box) end)
    |> Enum.map(&get_box_value/1)
    |> Enum.sum()

  end
end
