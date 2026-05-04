defmodule ExAoc.Day11Test do
  use ExUnit.Case, async: true

  describe "part1/1" do
    @tag :real
    test "real input" do
      input = ExAoc.read_input(11)
      assert ExAoc.Day11.part1(input) == 555
    end
  end

  describe "part2/1" do
    test "sample input" do
      input = ExAoc.read_sample_input(11)
      assert ExAoc.Day11.part2(input) == 2
    end

    @tag :real
    test "real input" do
      input = ExAoc.read_input(11)
      assert ExAoc.Day11.part2(input) == 502_447_498_690_860
    end
  end
end
