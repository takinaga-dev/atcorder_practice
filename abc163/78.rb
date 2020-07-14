N, c1, c2 = STDIN.gets.split.map(&:to_i);
result = 0
count = 0
N.times do |i|
  v = STDIN.gets.to_i
  if i == N - 1 then
    result += count * v
  elsif c1 >= v then
    result -= v
    count += 1
  elsif c2 <= v then
    result += v * count
    count = 0
  end
end

puts result
