defmodule ExAoc.Day11 do
  @moduledoc """
  Advent of Code 2025 - Day 11: Reactor

  Part 1: Count all distinct paths from "you" to "out".
  Part 2: Count paths from "svr" to "out" that visit both "dac" and "fft".
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
  Solve part 2: count paths from "svr" to "out" visiting both "dac" and "fft".
  """
  def part2(input) do
    graph = parse(input)
    # Track visited required nodes as a bitmask: bit 0 = "dac", bit 1 = "fft"
    {count, _cache} = count_paths_with_required(graph, "svr", 0, %{})
    count
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

  # --- Part 1: simple path counting ---

  defp count_paths(_graph, "out", cache), do: {1, cache}

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

  # --- Part 2: path counting with required node tracking ---

  # Reached "out" — only count if both required nodes visited (mask == 3)
  defp count_paths_with_required(_graph, "out", mask, cache) do
    count = if mask == 3, do: 1, else: 0
    {count, cache}
  end

  defp count_paths_with_required(graph, node, mask, cache) do
    # Update mask if we're at a required node
    mask = update_mask(node, mask)

    key = {node, mask}

    case Map.fetch(cache, key) do
      {:ok, count} ->
        {count, cache}

      :error ->
        targets = Map.get(graph, node, [])

        {total, cache} =
          Enum.reduce(targets, {0, cache}, fn target, {acc, cache} ->
            {count, cache} = count_paths_with_required(graph, target, mask, cache)
            {acc + count, cache}
          end)

        {total, Map.put(cache, key, total)}
    end
  end

  defp update_mask("dac", mask), do: Bitwise.bor(mask, 1)
  defp update_mask("fft", mask), do: Bitwise.bor(mask, 2)
  defp update_mask(_node, mask), do: mask
end
