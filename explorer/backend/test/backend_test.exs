defmodule BackendTest do
  use ExUnit.Case
  doctest Backend

  test "greets the world" do
    assert Backend.hello() == :world
  end
end
