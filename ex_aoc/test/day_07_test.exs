defmodule ExAoc.Day07Test do
  use ExUnit.Case, async: true

  describe "part1/1" do
    test "sample input" do
      input = ExAoc.read_sample_input(7)
      assert ExAoc.Day07.part1(input) == 21
    end

    @tag :real
    test "real input" do
      input = ExAoc.read_input(7)
      assert ExAoc.Day07.part1(input) == 1605
    end
  end

  describe "part2/1" do
    test "sample input" do
      input = ExAoc.read_sample_input(7)
      assert ExAoc.Day07.part2(input) == 40
    end

    @tag :real
    test "real input" do
      input = ExAoc.read_input(7)
      assert ExAoc.Day07.part2(input) == "expected"
    end
  end
end
