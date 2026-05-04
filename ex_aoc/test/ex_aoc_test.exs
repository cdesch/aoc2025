defmodule ExAocTest do
  use ExUnit.Case
  doctest ExAoc

  test "greets the world" do
    assert ExAoc.hello() == :world
  end
end
