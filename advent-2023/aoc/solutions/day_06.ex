defmodule DAY06 do
  def get_races() do
    [times, distances] = File.read!("inputs/day_06.input")
    |> String.split("\n")
    |> Enum.map(fn l ->
      List.last((String.split(l, ":")))
      |> String.split(" ")
      |> Enum.filter(fn el -> String.length(el) !== 0 end)
      |> Enum.map(&String.to_integer/1)
    end)
    Enum.zip(times, distances)
  end


  def get_races2() do
    [time, distance] = File.read!("inputs/day_06.input")
    |> String.split("\n")
    |> Enum.map(fn l ->
      List.last((String.split(l, ":")))
      |> String.replace(~r/\D/, "")
      |> String.to_integer
    end)
    {time, distance}
  end

  def get_race_wins({time, min_distance}) do
      h = :math.floor((time + :math.sqrt(:math.pow(time,2) - 4*(min_distance+1))) / 2)
      h2 = :math.ceil((time - :math.sqrt(:math.pow(time,2) - 4*(min_distance+1))) / 2)
      Kernel.abs(h - h2) + 1
  end

  def first_star() do
    get_races()
    |> Enum.map(fn r -> get_race_wins(r) end)
    |> Enum.reduce(1, fn el, acc -> el * acc end)
  end

  def second_star() do
    get_race_wins(get_races2())

  end
end
