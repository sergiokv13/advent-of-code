defmodule DAY22 do
  def parse_input() do
    File.read!("inputs/day_22.input")
    |> String.split("\n")
    |> Enum.map(fn row ->
      String.split(row, "~")
      |> Enum.map(
        fn pos ->
          String.split(pos, ",")
          |> Enum.map(&String.to_integer/1)
        end
      )
    end)
    |> Enum.sort_by(fn [[_,_,pz], [_,_,qz]] -> min(pz, qz) end, :desc)
  end

def on_segment([px, py, _], [qx, qy, _], [rx, ry, _]), do: (
  qx <= max(px, rx) && qx >= min(px, rx) &&
  qy <= max(py, ry) && qy >= min(py, ry)
)

  @spec orientation([...], [...], [...]) :: 0 | 1 | 2
  def orientation([px, py, _], [qx, qy, _], [rx, ry, _]) do
    val = (qy - py) * (rx - qx) - (qx - px) * (ry - qy);
    cond do
      val == 0 -> 0 # collinear
      val > 0 -> 1 # clockwise
      val < 0 -> 2 # counterclockwise
    end
  end

  def overlap([p1, q1], [p2, q2]) do
    o1 = orientation(p1, q1, p2)
    o2 = orientation(p1, q1, q2)
    o3 = orientation(p2, q2, p1)
    o4 = orientation(p2, q2, q1)

    cond do
      (o1 != o2 && o3 != o4)  -> true
      (o1 == 0 && on_segment(p1, p2, q1)) -> true
      (o2 == 0 && on_segment(p1, q2, q1)) -> true
      (o3 == 0 && on_segment(p2, p1, q2)) -> true
      (o4 == 0 && on_segment(p2, q1, q2)) -> true
      true -> false
    end
  end

  def collapse([]), do: []
  def collapse(cubes) do
    [current | rest] = cubes
    rest = collapse(rest)

    next_z =
      Enum.filter(rest, fn el -> overlap(current, el) end)
      |> Enum.map(fn [[_,_,pz],[_,_,qz]] -> max(pz, qz) end)
      |> (fn els -> if Enum.empty?(els), do: 0, else: Enum.max(els) end).()

    [[cpx, cpy, cpz], [cqx, cqy, cqz]] = current

    new_current = cond do
      cpz == cqz -> [[cpx, cpy, next_z + 1], [cqx, cqy, next_z + 1]]
      cpz < cqz -> [[cpx, cpy, next_z + 1], [cqx, cqy, (cqz - cpz) + next_z + 1]]
      cpz > cqz -> [[cpx, cpy, (cpz - cqz) + next_z + 1], [cqx, cqy, next_z + 1]]
    end

    [new_current] ++ rest
  end

  def get_below(current, collapsed_cubes) do
    [[_,_,cpz],[_,_,cqz]] = current
    min_current_z = min(cpz, cqz)

    Enum.filter(collapsed_cubes, fn
      el ->
        overlapped = overlap(current, el)
        [[_, _, pz], [_, _, qz]] = el
        max_el_z = max(pz, qz)
        overlapped && max_el_z + 1 == min_current_z
    end)
  end

  def at_bottom([[_,_,pz], [_,_,qz]]), do: pz == 1 || qz == 1

  def get_below_above(cubes) do
    below_cube = cubes
    |> Enum.into(%{}, fn el -> {el, get_below(el, cubes)} end)

    above_cube = Map.to_list(below_cube)
    |> Enum.reduce(%{}, fn {cube, below_cubes}, acc ->
      Enum.reduce(below_cubes, acc, fn child, iacc ->
        curr = Map.get(iacc, child)
        case curr do
          nil -> Map.put(iacc, child, [cube])
          _ -> Map.put(iacc, child, curr ++ [cube])
        end
      end)
    end)

    {below_cube, above_cube}
  end

  def first_star() do
    cubes = parse_input() |> collapse
    {below_cube, _} = get_below_above(cubes)

    Enum.reduce(cubes, 0, fn cube, count ->
      can_be_removed = cubes
      |> Enum.reject(fn c -> c == cube end)
      |> Enum.reduce(true, fn other_cube, acc ->
        deps = Map.get(below_cube, other_cube)

        cond do
          at_bottom(other_cube) -> acc && true
          true -> if (Enum.reject(deps, fn d -> d == cube end)
                  |> Enum.empty?), do: false, else: acc && true
        end
      end)
      if (can_be_removed), do: count + 1, else: count
    end)
  end

  def second_star() do
    cubes = parse_input()
    |> collapse

    Enum.map(cubes, fn cube ->
      filtered_cubes = Enum.reject(cubes, fn c -> c == cube end)

      collapse(filtered_cubes)
      |> Enum.zip(filtered_cubes)
      |> Enum.reduce(0, fn {c,c2}, acc -> if (c == c2), do: acc + 0, else: acc + 1 end)
    end)
    |> Enum.sum
  end
end
