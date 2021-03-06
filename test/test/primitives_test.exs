defmodule PrimitivesTest do
  use ExUnit.Case, async: true

  test "number decoding and encoding" do
    assert 3 == TestNative.add_u32(1, 2)
    assert 3 == TestNative.add_i32(6, -3)
    assert -3 == TestNative.add_i32(3, -6)
  end

  test "number decoding should fail on invalid terms" do
    assert_raise ArgumentError, fn -> TestNative.add_u32(-1, 1) end
    assert_raise ArgumentError, fn -> TestNative.add_u32("1", 1) end
    assert_raise ArgumentError, fn -> TestNative.add_i32(2147483648, 1) end
  end
end
