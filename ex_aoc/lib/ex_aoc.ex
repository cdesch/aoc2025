defmodule ExAoc do
  @moduledoc """
  Advent of Code 2025 solutions in Elixir.
  """

  @doc """
  Resolve the path to an input file for a given day.
  """
  def input_path(day, sample \\ false) do
    padded = day |> Integer.to_string() |> String.pad_leading(2, "0")

    filename =
      if sample,
        do: "day_#{padded}_sample.txt",
        else: "day_#{padded}.txt"

    Path.join([File.cwd!(), "input", filename])
  end

  @doc """
  Read the full puzzle input for a given day.
  """
  def read_input(day) do
    day |> input_path() |> File.read!() |> String.trim_trailing()
  end

  @doc """
  Read the sample input for a given day.
  """
  def read_sample_input(day) do
    day |> input_path(true) |> File.read!() |> String.trim_trailing()
  end

  @doc """
  Run a day's solution against the real input.
  """
  def run(day) do
    module = day_module(day)
    input = read_input(day)
    padded = day |> Integer.to_string() |> String.pad_leading(2, "0")

    IO.puts("=== Day #{padded} ===")
    IO.puts("Part 1: #{module.part1(input)}")
    IO.puts("Part 2: #{module.part2(input)}")
  end

  defp day_module(day) do
    padded = day |> Integer.to_string() |> String.pad_leading(2, "0")
    Module.concat(ExAoc, "Day#{padded}")
  end
end
