#!/usr/bin/env ruby

filename = ARGV[0]

hex = IO.binread(filename)
  .bytes
  .map { |byte| "0x#{byte.to_s(16).upcase}" }
  .join("\n")

puts hex
