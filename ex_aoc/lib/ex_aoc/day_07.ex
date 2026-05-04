defmodule ExAoc.Day07 do
  @moduledoc """
  Advent of Code 2025 - Day 07: Laboratories

  A tachyon beam enters at S and moves downward. When it hits a splitter (^),
  it stops and two new beams continue from the left and right of the splitter.
  Beams in the same column merge (their timeline counts add up).
  """

  @doc """
  Solve part 1: count how many times a beam is split.
  """
  def part1(input) do
    {splits, _beams} = simulate(input)
    splits
  end

  @doc """
  Solve part 2: count total timelines (sum of all beam counts at the bottom).
  """
  def part2(input) do
    {_splits, beams} = simulate(input)
    Enum.sum(beams)
  end

  defp simulate(input) do
    [first_row | rest] = String.split(input, "\n")
    width = String.length(first_row)
    start = find_start(first_row)

    # beams tracks timeline counts per column
    beams = List.duplicate(0, width) |> List.replace_at(start, 1)

    Enum.reduce(rest, {0, beams}, fn row, {splits, beams} ->
      row
      |> String.graphemes()
      |> Enum.with_index()
      |> Enum.reduce({splits, beams}, fn {char, col}, {splits, beams} ->
        count = Enum.at(beams, col)

        if char == "^" and count > 0 do
          beams =
            beams
            |> add_at(col - 1, count, width)
            |> add_at(col + 1, count, width)
            |> List.replace_at(col, 0)

          {splits + 1, beams}
        else
          {splits, beams}
        end
      end)
    end)
  end

  defp find_start(row) do
    row
    |> String.graphemes()
    |> Enum.find_index(&(&1 == "S"))
  end

  defp add_at(list, index, amount, width) when index >= 0 and index < width do
    List.update_at(list, index, &(&1 + amount))
  end

  defp add_at(list, _index, _amount, _width), do: list
end
