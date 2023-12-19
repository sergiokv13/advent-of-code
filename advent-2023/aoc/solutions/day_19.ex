defmodule DAY19 do

  def parse_items(items) do
    items
        |> String.split("\n")
        |> Enum.map(fn row ->
          String.split(row, ",")
          |> Enum.reduce(%{}, fn el, acc ->
            [var, value] = el |> String.replace("{", "") |> String.replace("}", "") |> String.split("=")
            Map.put(acc, var, String.to_integer(value))
          end)
        end)
  end

  def parse_workflow(workflow) do
      [key, raw_wf] = String.split(workflow, "{")

      {key, raw_wf
        |> String.replace("}", "")
        |> String.split(",")
        |> Enum.map(fn ins ->
          splitted = String.split(ins, ":")
          if (Enum.count(splitted) == 1) do
            [nil, "|", nil, hd(splitted)]
          else
            [ins, dest] = String.split(ins, ":")
            [v, v2] = String.split(ins, ~r/[<>]/)
            [
              v,
              if (String.contains?(ins, ">")) do ">" else "<" end,
              String.to_integer(v2),
              dest
            ]
          end
        end)
      }
  end

  def parse_input() do
    [workflows, items] = File.read!("inputs/day_19.input")
    |> String.split("\n\n")

    {
      workflows
        |> String.split("\n")
        |> Enum.map(&parse_workflow/1)
        |> Enum.into(%{}),
        parse_items(items)
    }
  end

  def check_instruction(values, instructions) do
    Enum.reduce(instructions, nil, fn el, res ->
      [key, operator, value, dest_res] = el
      cond do
        res != nil -> res
        operator == ">" && Map.get(values, key) > value -> dest_res
        operator == "<" && Map.get(values, key) < value -> dest_res
        operator == "|" -> dest_res
        true -> res
      end
    end)
  end
  def perform_workflow(_,_,"R"), do: 0
  def perform_workflow(item,_,"A"), do: Map.values(item) |> Enum.sum
  def perform_workflow(item, workflow, curr) do
    new_curr = check_instruction(item, Map.get(workflow, curr))
    perform_workflow(item, workflow, new_curr)
  end

  def merge_ranges(r1, r2) do
    Enum.reduce(["x", "m", "a", "s"], %{}, fn key, acc ->
      (r1i..r1e)= Map.get(r1,key, (1..4000))
      (r2i..r2e) = Map.get(r2,key, (1..4000))
      Map.put(acc, key, (max(r1i, r2i)..min(r1e, r2e)))
    end)
  end

  def make_all_fail(_, -1), do: %{}
  def make_all_fail(instructions, idx) do
    curr_instruction = Enum.at(instructions, idx)
    [key, operator, value, _] = curr_instruction

    instruction_fail = case operator do
      ">" -> %{key => (1..value)}
      "<" -> %{key => (value..4000)}
    end

    merge_ranges(instruction_fail, make_all_fail(instructions, idx - 1))
  end

  def get_working_ranges_for(instructions, target) do
    Enum.with_index(instructions)
      |> Enum.filter(fn {[_, _, _, v], _} -> v == target end)
      |> Enum.map(fn {[key, operator, value, _], idx} ->
        until_condition = make_all_fail(instructions, idx - 1)
        condition = cond do
          operator == ">" -> %{key => (value+1..4000)}
          operator == "<" -> %{key => (1..value-1)}
          operator == "|" -> %{key => (1..4000)}
        end

        merge_ranges(until_condition, condition)
      end)
  end

  def trace_back(_, "in"), do: [%{}]
  def trace_back(workflows, current) do
    Enum.to_list(workflows)
    |> Enum.map(fn {key, instructions} ->
      { key, get_working_ranges_for(instructions, current)}
    end)
    |> Enum.filter(fn {_, items} -> !Enum.empty?(items) end)
    |> Enum.flat_map(fn {key, items} ->
      items_to_get_here = trace_back(workflows, key)
      for item1 <- items, item2 <- items_to_get_here do
        merge_ranges(item1, item2)
      end
    end)
  end

  def first_star() do
    {workflows, items} = parse_input()
    items |> Enum.map(&perform_workflow(&1, workflows, "in")) |> Enum.sum
  end

  def second_star() do
    {workflows, _} = parse_input()

    trace_back(workflows, "A")
    |> Enum.map(fn item_ranges ->
      item_ranges
        |> Map.values
        |> Enum.map(fn (i..e) -> e+1 - i end)
        |> Enum.reduce(1, fn el, acc -> el * acc end)
    end)
    |> Enum.sum

  end
end
