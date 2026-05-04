defmodule ExAoc.Day11 do
  @moduledoc """
  Advent of Code 2025 - Day 11: Reactor

  Given a directed graph of devices, count all distinct paths from "you" to "out".
  Uses memoized DFS (dynamic programming on a DAG).
  """

  @doc """
  Solve part 1: count paths from "you" to "out".
  """
  def part1(input) do
    graph = parse(input)
    {count, _cache} = count_paths(graph, "you", %{})
    count
  end

  @doc """
  Solve part 2.
  """
  def part2(_input) do
    "not implemented"
  end

  defp parse(input) do
    input
    |> String.split("\n")
    |> Enum.reduce(%{}, fn line, graph ->
      [device, outputs] = String.split(line, ": ")
      targets = String.split(outputs, " ")
      Map.put(graph, device, targets)
    end)
  end

  # Base case: we've reached "out"
  defp count_paths(_graph, "out", cache), do: {1, cache}

  # Memoized recursive case
  defp count_paths(graph, node, cache) do
    case Map.fetch(cache, node) do
      {:ok, count} ->
        {count, cache}

      :error ->
        targets = Map.get(graph, node, [])

        {total, cache} =
          Enum.reduce(targets, {0, cache}, fn target, {acc, cache} ->
            {count, cache} = count_paths(graph, target, cache)
            {acc + count, cache}
          end)

        {total, Map.put(cache, node, total)}
    end
  end
end
