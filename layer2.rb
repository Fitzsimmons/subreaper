
pid = fork do
  sleep 4
  exit 20
end

puts pid
