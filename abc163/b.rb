N,M = STDIN.gets.split.map(&:to_i)
A = STDIN.gets.split.map(&:to_i)

res = N - A.sum
if res < 0 then
  puts '-1'
else
  puts res
end
