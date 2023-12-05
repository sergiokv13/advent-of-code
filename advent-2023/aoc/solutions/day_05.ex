defmodule DAY05 do
  def parse_input() do
    [seeds | raw_maps] = File.read!("inputs/day_05.input") |> String.split("\n")
    seeds = seeds
      |> String.replace("seeds: ", "")
      |> String.split(" ")
      |> Enum.map(&String.to_integer/1)

    maps = raw_maps
      |> Enum.reject(fn s -> String.length(s) == 0 end)
      |> Enum.chunk_by(fn el -> String.contains?(el, "map:") end)
      |> Enum.reject(fn el -> String.contains?(hd(el), "map:") end)
      |> Enum.map(fn r ->
        Enum.map(r, fn el ->
          el
          |> String.split(" ")
          |> Enum.map(&String.to_integer/1)
        end)
        |> Enum.map(fn [d,s,size] -> {(s..(s + size - 1)), (d..(d + size - 1))} end)
      end)

    {seeds, maps}
  end

  def get_destination(seed, []), do: seed
  def get_destination(seed, [range | tail]) do
    { source, destination } = range
    if seed in source do
      di.._ = destination
      si.._ = source
      di + (seed - si)
    else
      get_destination(seed, tail)
    end
  end

  def get_intersection(seedi..seede, bi..be, di..de) do
    ii..ie = max(seedi, bi)..min(seede, be)
    if ie >= ii, do: [ii..ie, (ii-bi + di)..(ie-be + de)], else: nil
  end

  def exclude_intersections(seedi..seede, intersections) do
    ([
      seedi |
      Enum.flat_map(Enum.sort(intersections), fn i..e -> [i,e] end)]
      ++ [seede]
    ) |> Enum.chunk_every(2)
    |> Enum.map(fn [x,y] -> (if x != seedi, do: x+1, else: x)..(if y != seede, do: y-1, else: y) end)
    |> Enum.reject(fn x..y -> x > y end)
  end

  def get_new_seeds(seed, map) do
    intersections = Enum.map(map, fn {source, dest} -> get_intersection(seed, source, dest) end)
    |> Enum.reject(fn x -> x == nil end)

    source_intersections = Enum.map(intersections, fn [in_source, _] -> in_source end)
    exclude_intersections(seed, source_intersections) ++ Enum.map(intersections, fn [_, in_dest] -> in_dest end)
  end

  def navigate(rseeds, maps) do
    Enum.reduce(maps, rseeds, fn map, acc ->
      Enum.flat_map(acc, fn seed -> get_new_seeds(seed,map) end)
    end)
  end

  def first_star() do
    {seeds, maps} = parse_input()
    Enum.map(seeds, fn seed ->
      maps
      |> Enum.reduce(seed, fn ranges, acc ->
        get_destination(acc, ranges)
      end)
    end)
    |> Enum.min()
  end

  def second_star() do
    { seeds, maps } = parse_input()

    seeds = Enum.chunk_every(seeds,2)
    |> Enum.map(fn [start, size] -> start..(start+size-1) end)\

    navigate(seeds, maps)
    |> Enum.map(fn si.._ -> si end)
    |> Enum.min()

  end
end
