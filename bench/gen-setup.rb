#!/usr/bin/env ruby

File.open(File.join(__dir__, 'setup.sql'), 'w') do |f|
  f.puts <<~SQL
    drop table if exists benchmark_tests;
    create table benchmark_tests (
      id bigint primary key auto_increment
      , name varchar(256) not null
      , age integer
      , created_at datetime(6) not null default current_timestamp(6)
    );
  SQL

  1000000.times.each_slice(10000) do |xs|
    f.puts "insert into benchmark_tests (name, age) values"
    xs.each_with_index do |x, i|
      name = "name-#{x}"
      age = x % 100 == 0 ? 'null' : x % 100
      f.puts "#{i == 0 ? '  ' : '  , '}('#{name}', #{age})"
    end
    f.puts ';'
  end
end
