defmodule DAY18 do
  def get_instructions() do
    File.read!("inputs/day_18.input")
    |> String.split("\n")
    |> Enum.map(&String.split/1)
    |> Enum.map(fn [dir, v, code] ->
      {
        dir,
        String.to_integer(v),
        code
          |> String.replace(")", "")
          |> String.replace("(", "")
          |> String.replace("#", "")
      }
    end)
  end

  def get_points1([], _), do: []
  def get_points1(instructions, {y,x}) do
    [{dir, v, _} | rest] = instructions

    next_pos = case dir do
      "R" -> {y, x+v}
      "L" -> {y, x-v}
      "D" -> {y+v, x}
      "U" -> {y-v, x}
    end

   [next_pos] ++ get_points1(rest, next_pos)
  end

  def det({y1,x1}, {y2, x2}), do: (y1 * x2) - (y2 * x1)

  def get_area(instructions) do
    points = get_points1(instructions, {0,0})
    edges = points
    |> Enum.reverse()
    |> Enum.chunk_every(2, 1)
    |> Enum.filter(fn x -> Enum.count(x) > 1 end)

    # Gauss area
    area = edges
      |> Enum.map(fn [f,s] -> det(f,s) end)
      |> Enum.sum()
      |> Kernel./(2.0)


    perimeter = edges
      |> Enum.map(fn [{y1,x1}, {y2,x2}] -> abs(y1-y2) + abs(x1-x2) end)
      |> Enum.sum()
      |> Kernel.+(
        (fn ->
          {y1,x1} = hd(points)
          {y2,x2} = List.last(points)
          abs(y1-y2) + abs(x1-x2)
        end).()
      )

      # do not include perimeter in area twice
      # offset by one not sure why
      area + (perimeter / 2) + 1
  end
  def first_star() do
    get_instructions() |> get_area
  end

  def second_star() do
    get_instructions()
    |> Enum.map(fn {_, _, code} ->
      {
        case String.slice(code, 5, 6) do
          "0" -> "R"
          "1" -> "D"
          "2" -> "L"
          "3" -> "U"
        end,
        String.to_integer(String.slice(code, 0, 5), 16),
        code
      }
    end)
    |> get_area
  end
end
