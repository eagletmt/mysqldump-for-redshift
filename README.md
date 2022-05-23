# mysqldump-for-redshift
mysqldump for Redshift

## Benchmarking with mys3dump
CPU: AMD Ryzen 9 3900X 12-Core Processor

```
% MYS3DUMP_JAR_PATH=~/.clg/github.com/bricolages/mys3dump/build/libs/mys3dump-1.2.1-all.jar bench/run.sh
+ '[' -z /home/eagletmt/.clg/github.com/bricolages/mys3dump/build/libs/mys3dump-1.2.1-all.jar ']'
+ cargo build --release
    Finished release [optimized] target(s) in 0.08s
+ mysql_user=m4r
+ mysql_password=m4r
+ mysql_database=m4r
+ mysql_table=benchmark_tests
+ s3_bucket=eagletmt-test-bucket
+ mysql -h 127.0.0.1 -u m4r -pm4r -D m4r --batch
+ cargo run --release -- -h localhost -P 3306 -u m4r -p m4r -D m4r -t benchmark_tests -b eagletmt-test-bucket -x m4r/benchmark_tests/ -d -w 1 -C
    Finished release [optimized] target(s) in 0.08s
     Running `/home/eagletmt/.cargo/target/release/mysqldump-for-redshift -h localhost -P 3306 -u m4r -p m4r -D m4r -t benchmark_tests -b eagletmt-test-bucket -x m4r/benchmark_tests/ -d -w 1 -C`
2022-05-23T17:06:37.650991Z  WARN mysqldump_for_redshift: write_concurrency option is given but it has no effect on mysqldump-for-redshift
2022-05-23T17:06:38.071897Z  INFO mysqldump_for_redshift: Delete object: s3://eagletmt-test-bucket/m4r/benchmark_tests/00000.json.gz
2022-05-23T17:06:38.108715Z  INFO mysqldump_for_redshift: Send query to MySQL: select `id`, `name`, `age`, `created_at` from `benchmark_tests`
2022-05-23T17:06:42.185075Z  WARN sqlx::query: select `id`, `name`, `age`, …; rows affected: 0, rows returned: 1000000, elapsed: 4.073s

select
  `id`,
  `name`,
  `age`,
  `created_at`
from
  `benchmark_tests`

2022-05-23T17:06:42.185368Z  INFO mysqldump_for_redshift: Uploading to s3://eagletmt-test-bucket/m4r/benchmark_tests/00000.json.gz (5504928 bytes)

real    0m5.097s
user    0m4.206s
sys     0m0.111s
+ java -jar /home/eagletmt/.clg/github.com/bricolages/mys3dump/build/libs/mys3dump-1.2.1-all.jar -h localhost -P 3306 -u m4r -p m4r -D m4r -t benchmark_tests -b eagletmt-test-bucket -x mys3dump/benchmark_tests/ -d -w 1 -C
2022-05-24 02:06:42,833 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Connecting to: jdbc:mysql://localhost:3306/m4r?null
2022-05-24 02:06:42,902 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Execute query for metadata: SELECT `id`,`name`,`age`,`created_at` FROM `benchmark_tests` LIMIT 1
2022-05-24 02:06:42,902 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Connecting to: jdbc:mysql://localhost:3306/m4r?null
2022-05-24 02:06:42,904 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Query returned: SELECT `id`,`name`,`age`,`created_at` FROM `benchmark_tests` LIMIT 1
2022-05-24 02:06:42,911 [main]  INFO: org.bricolages.mys3dump.TimeZonePreprocessOperation: Init TimeZonePreprocessOperation: src-zone-offset=Z, dst-zone-offset=Z
2022-05-24 02:06:48,339 [main]  INFO: org.bricolages.mys3dump.S3OutputLocation: Delete object: s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00000.json.gz
2022-05-24 02:06:48,501 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Start Dump.
2022-05-24 02:06:48,503 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Reader thread number: 1
2022-05-24 02:06:48,503 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Writer thread number: 1
2022-05-24 02:06:48,510 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLDataSource: Connecting to: jdbc:mysql://localhost:3306/m4r?null
2022-05-24 02:06:48,516 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLProducer: [SQL] set net_write_timeout = 600
2022-05-24 02:06:48,518 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLProducer: Execute query: SELECT `id`,`name`,`age`,`created_at` FROM `benchmark_tests`
2022-05-24 02:06:48,522 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLProducer: Query returned
2022-05-24 02:06:52,060 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Total read rows: 1000000
2022-05-24 02:06:54,776 [pool-4-thread-1]  INFO: org.bricolages.mys3dump.S3OutputStream: S3 object created (5548028 bytes): s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00000.json.gz
2022-05-24 02:06:54,779 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Total write rows: 1000000
2022-05-24 02:06:54,780 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Dump finished.

real    0m12.188s
user    0m6.955s
sys     0m0.778s
+ java -jar /home/eagletmt/.clg/github.com/bricolages/mys3dump/build/libs/mys3dump-1.2.1-all.jar -h localhost -P 3306 -u m4r -p m4r -D m4r -t benchmark_tests -b eagletmt-test-bucket -x mys3dump/benchmark_tests/ -d -w 2 -C
2022-05-24 02:06:55,020 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Connecting to: jdbc:mysql://localhost:3306/m4r?null
2022-05-24 02:06:55,090 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Execute query for metadata: SELECT `id`,`name`,`age`,`created_at` FROM `benchmark_tests` LIMIT 1
2022-05-24 02:06:55,090 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Connecting to: jdbc:mysql://localhost:3306/m4r?null
2022-05-24 02:06:55,092 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Query returned: SELECT `id`,`name`,`age`,`created_at` FROM `benchmark_tests` LIMIT 1
2022-05-24 02:06:55,099 [main]  INFO: org.bricolages.mys3dump.TimeZonePreprocessOperation: Init TimeZonePreprocessOperation: src-zone-offset=Z, dst-zone-offset=Z
2022-05-24 02:07:00,320 [main]  INFO: org.bricolages.mys3dump.S3OutputLocation: Delete object: s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00000.json.gz
2022-05-24 02:07:00,478 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Start Dump.
2022-05-24 02:07:00,480 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Reader thread number: 1
2022-05-24 02:07:00,480 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Writer thread number: 2
2022-05-24 02:07:00,488 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLDataSource: Connecting to: jdbc:mysql://localhost:3306/m4r?null
2022-05-24 02:07:00,494 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLProducer: [SQL] set net_write_timeout = 600
2022-05-24 02:07:00,495 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLProducer: Execute query: SELECT `id`,`name`,`age`,`created_at` FROM `benchmark_tests`
2022-05-24 02:07:00,499 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLProducer: Query returned
2022-05-24 02:07:03,650 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Total read rows: 1000000
2022-05-24 02:07:06,370 [pool-4-thread-1]  INFO: org.bricolages.mys3dump.S3OutputStream: S3 object created (3099354 bytes): s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00001.json.gz
2022-05-24 02:07:06,376 [pool-4-thread-2]  INFO: org.bricolages.mys3dump.S3OutputStream: S3 object created (3091846 bytes): s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00000.json.gz
2022-05-24 02:07:06,377 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Total write rows: 1000000
2022-05-24 02:07:06,377 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Dump finished.

real    0m11.598s
user    0m7.834s
sys     0m0.863s
+ java -jar /home/eagletmt/.clg/github.com/bricolages/mys3dump/build/libs/mys3dump-1.2.1-all.jar -h localhost -P 3306 -u m4r -p m4r -D m4r -t benchmark_tests -b eagletmt-test-bucket -x mys3dump/benchmark_tests/ -d -w 3 -C
2022-05-24 02:07:06,617 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Connecting to: jdbc:mysql://localhost:3306/m4r?null
2022-05-24 02:07:06,684 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Execute query for metadata: SELECT `id`,`name`,`age`,`created_at` FROM `benchmark_tests` LIMIT 1
2022-05-24 02:07:06,684 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Connecting to: jdbc:mysql://localhost:3306/m4r?null
2022-05-24 02:07:06,686 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Query returned: SELECT `id`,`name`,`age`,`created_at` FROM `benchmark_tests` LIMIT 1
2022-05-24 02:07:06,693 [main]  INFO: org.bricolages.mys3dump.TimeZonePreprocessOperation: Init TimeZonePreprocessOperation: src-zone-offset=Z, dst-zone-offset=Z
2022-05-24 02:07:11,914 [main]  INFO: org.bricolages.mys3dump.S3OutputLocation: Delete object: s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00000.json.gz
2022-05-24 02:07:11,914 [main]  INFO: org.bricolages.mys3dump.S3OutputLocation: Delete object: s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00001.json.gz
2022-05-24 02:07:12,126 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Start Dump.
2022-05-24 02:07:12,128 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Reader thread number: 1
2022-05-24 02:07:12,128 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Writer thread number: 3
2022-05-24 02:07:12,136 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLDataSource: Connecting to: jdbc:mysql://localhost:3306/m4r?null
2022-05-24 02:07:12,143 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLProducer: [SQL] set net_write_timeout = 600
2022-05-24 02:07:12,144 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLProducer: Execute query: SELECT `id`,`name`,`age`,`created_at` FROM `benchmark_tests`
2022-05-24 02:07:12,147 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLProducer: Query returned
2022-05-24 02:07:17,137 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Total read rows: 1000000
2022-05-24 02:07:19,747 [pool-4-thread-3]  INFO: org.bricolages.mys3dump.S3OutputStream: S3 object created (1996380 bytes): s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00001.json.gz
2022-05-24 02:07:19,747 [pool-4-thread-2]  INFO: org.bricolages.mys3dump.S3OutputStream: S3 object created (2220907 bytes): s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00000.json.gz
2022-05-24 02:07:19,811 [pool-4-thread-1]  INFO: org.bricolages.mys3dump.S3OutputStream: S3 object created (2218901 bytes): s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00002.json.gz
2022-05-24 02:07:19,812 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Total write rows: 1000000
2022-05-24 02:07:19,812 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Dump finished.

real    0m13.436s
user    0m9.065s
sys     0m1.163s
+ java -jar /home/eagletmt/.clg/github.com/bricolages/mys3dump/build/libs/mys3dump-1.2.1-all.jar -h localhost -P 3306 -u m4r -p m4r -D m4r -t benchmark_tests -b eagletmt-test-bucket -x mys3dump/benchmark_tests/ -d -w 4 -C
2022-05-24 02:07:20,057 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Connecting to: jdbc:mysql://localhost:3306/m4r?null
2022-05-24 02:07:20,127 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Execute query for metadata: SELECT `id`,`name`,`age`,`created_at` FROM `benchmark_tests` LIMIT 1
2022-05-24 02:07:20,127 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Connecting to: jdbc:mysql://localhost:3306/m4r?null
2022-05-24 02:07:20,130 [main]  INFO: org.bricolages.mys3dump.MySQLDataSource: Query returned: SELECT `id`,`name`,`age`,`created_at` FROM `benchmark_tests` LIMIT 1
2022-05-24 02:07:20,136 [main]  INFO: org.bricolages.mys3dump.TimeZonePreprocessOperation: Init TimeZonePreprocessOperation: src-zone-offset=Z, dst-zone-offset=Z
2022-05-24 02:07:25,335 [main]  INFO: org.bricolages.mys3dump.S3OutputLocation: Delete object: s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00000.json.gz
2022-05-24 02:07:25,335 [main]  INFO: org.bricolages.mys3dump.S3OutputLocation: Delete object: s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00001.json.gz
2022-05-24 02:07:25,335 [main]  INFO: org.bricolages.mys3dump.S3OutputLocation: Delete object: s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00002.json.gz
2022-05-24 02:07:25,789 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Start Dump.
2022-05-24 02:07:25,791 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Reader thread number: 1
2022-05-24 02:07:25,791 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Writer thread number: 4
2022-05-24 02:07:25,799 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLDataSource: Connecting to: jdbc:mysql://localhost:3306/m4r?null
2022-05-24 02:07:25,805 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLProducer: [SQL] set net_write_timeout = 600
2022-05-24 02:07:25,807 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLProducer: Execute query: SELECT `id`,`name`,`age`,`created_at` FROM `benchmark_tests`
2022-05-24 02:07:25,810 [pool-3-thread-1]  INFO: org.bricolages.mys3dump.MySQLProducer: Query returned
2022-05-24 02:07:30,801 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Total read rows: 1000000
2022-05-24 02:07:33,371 [pool-4-thread-2]  INFO: org.bricolages.mys3dump.S3OutputStream: S3 object created (1638290 bytes): s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00002.json.gz
2022-05-24 02:07:33,372 [pool-4-thread-1]  INFO: org.bricolages.mys3dump.S3OutputStream: S3 object created (1650806 bytes): s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00001.json.gz
2022-05-24 02:07:33,428 [pool-4-thread-3]  INFO: org.bricolages.mys3dump.S3OutputStream: S3 object created (1649380 bytes): s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00003.json.gz
2022-05-24 02:07:33,471 [pool-4-thread-4]  INFO: org.bricolages.mys3dump.S3OutputStream: S3 object created (1620743 bytes): s3://eagletmt-test-bucket/mys3dump/benchmark_tests/00000.json.gz
2022-05-24 02:07:33,472 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Total write rows: 1000000
2022-05-24 02:07:33,472 [main]  INFO: org.bricolages.mys3dump.MyS3Dump: Dump finished.

real    0m13.660s
user    0m9.555s
sys     0m1.859s
```
