defmodule ExAoc.DayXXTest do
  use ExUnit.Case, async: true

  describe "part1/1" do
    test "sample input" do
      input = ExAoc.read_sample_input(XX)
      assert ExAoc.DayXX.part1(input) == "expected"
    end

    @tag :real
    test "real input" do
      input = ExAoc.read_input(XX)
      assert ExAoc.DayXX.part1(input) == "expected"
    end
  end

  describe "part2/1" do
    test "sample input" do
      input = ExAoc.read_sample_input(XX)
      assert ExAoc.DayXX.part2(input) == "expected"
    end

    @tag :real
    test "real input" do
      input = ExAoc.read_input(XX)
      assert ExAoc.DayXX.part2(input) == "expected"
    end
  end
end
