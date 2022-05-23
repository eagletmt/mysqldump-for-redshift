#!/bin/bash
set -ex

if [ -z "$MYS3DUMP_JAR_PATH" ];then
  echo 'Set MYS3DUMP_JAR_PATH'
  exit 1
fi

cargo build --release

mysql_user=m4r
mysql_password=m4r
mysql_database=m4r
mysql_table=benchmark_tests
s3_bucket=eagletmt-test-bucket

mysql -h 127.0.0.1 -u $mysql_user -p$mysql_password -D $mysql_database --batch < bench/setup.sql

time cargo run --release -- -h localhost -P 3306 -u $mysql_user -p $mysql_password -D $mysql_database -t $mysql_table -b eagletmt-test-bucket -x m4r/benchmark_tests/ -d -w 1 -C
time java -jar "$MYS3DUMP_JAR_PATH" -h localhost -P 3306 -u $mysql_user -p $mysql_password -D $mysql_database -t $mysql_table -b eagletmt-test-bucket -x mys3dump/benchmark_tests/ -d -w 1 -C
