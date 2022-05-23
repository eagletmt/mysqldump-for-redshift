drop table if exists tests;
create table tests (
  col_boolean boolean
  , col_tinyint tinyint
  , col_smallint smallint
  , col_mediumint mediumint
  , col_int int
  , col_bigint bigint
  , col_float float
  , col_double double
  , col_utinyint tinyint unsigned
  , col_usmallint smallint unsigned
  , col_umediumint mediumint unsigned
  , col_uint int unsigned
  , col_ubigint bigint unsigned
  , col_ufloat float unsigned
  , col_udouble double unsigned
  , col_date date
  , col_time time
  , col_datetime datetime
  , col_timestamp timestamp
  , col_char char(3)
  , col_varchar varchar(3)
  , col_binary binary(3)
  , col_varbinary varbinary(3)
  , col_tinyblob tinyblob
  , col_blob blob
  , col_mediumblob mediumblob
  , col_longblob longblob
  , col_tinytext tinytext
  , col_text text
  , col_mediumtext mediumtext
  , col_longtext longtext
  , col_enum enum('e1', 'e2', 'e3')
  , col_set set('s1', 's2', 's3')
  , col_geometry geometry
  , col_json json
  , col_time6 time(6)
  , col_datetime6 datetime(6)
  , col_timestamp6 timestamp(6) not null default current_timestamp(6) on update current_timestamp(6)
);
insert into tests values (
  true
  , 2
  , 3
  , 4
  , 5
  , 6
  , 7.1
  , 8.2
  , 9
  , 10
  , 11
  , 12
  , 13
  , 14.3
  , 15.4
  , '2022-05-19'
  , '01:52:06'
  , '2022-05-19 01:53:32'
  , '2022-05-19 01:54:11'
  , '20'
  , '21'
  , '22'
  , '23'
  , '24'
  , '25'
  , '26'
  , '27'
  , '28'
  , '29'
  , '30'
  , '31'
  , 'e2'
  , 's1,s3'
  , ST_GeomFromText('POINT(34 0)')
  , '{"values": [35]}'
  , '07:34:48.609548'
  , '2022-05-23 07:15:09.982443'
  , '2022-05-23 07:15:23.331896'
);
