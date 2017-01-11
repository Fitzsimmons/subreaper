pid = fork do
  sleep 3
  exit 20
end

puts pid
