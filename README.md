# Source code Demo Report
## CHAPTER 6 : Building Native Applications with Delta Lake
### Python
`% cd ~/dldg # Choose the directory of your choice`
`% virtualenv venv # Configure a Python virtualenv for managing deps in the ./venv/ directory`
`% source ./venv/bin/activate # Activate the virtualenv in this shell`

`% pip install 'deltalake>=0.18.2' pandas`
`% python`
`>>> from deltalake import DeltaTable`
`>>> dt = DeltaTable('./deltatbl-partitioned')`
`>>> dt.files()`

`>>> df = dt.to_pandas()`
`>>> df`
- Reading large datasets
1. Collect references to the necessary data files—in essence, the .parquet files returned from dt.files().
2. Retrieve those data files from storage (the local filesystem in this example).
3. Deserialize and load those data files into memory.
4. Construct the pandas.DataFrame object using the data loaded in memory
  + Partitions : Structuring of data in storage to allow grouping of files by common prefixes, such as mytable/year=2024/*.parquet
    `>>> dt.to_pandas(partitions=[('c2', '=', 'foo0')])`
    `>>> dt.files([('c2', '=', 'foo0')])`
    `>>> dt.to_pandas(partitions=[('c2', '=', 'foo0')], columns=['c1'])`
    `>>> dt.to_pandas(partitions=[('c2', '=', 'foo0')], columns=['c1'], filters=[('c1', '<=', 4), ('c1', '>', 0)])`
  + File statistics : Additional metadata included by the writer in the transaction log about the .parquet file, whether Apache Spark or a native Python/Rust, that indicates the minimum or maximum values of columns contained in that data column.
    `>>> from deltalake import DeltaTable`
    `>>> dt = DeltaTable('./data/gen/filestats')`
    `>>> len(dt.files())`

    `>>> df = dt.to_pandas(filters=[('year', '=', 2022), ('month', '>=', 9)])`
    `>>> df`
- Writing data
  `>>> import pandas as pd`
  `>>> from deltalake import write_deltalake, DeltaTable`
  `>>> df = pd.read_csv('./data/co2_mm_mlo.csv', comment='#')`
  `>>> len(df)`
  
  `>>> write_deltalake('./data/co2_monthly', df)`
  `>>> dt = DeltaTable('./data/co2_monthly')`
  `>>> dt.files()`
  
  `>>> df = dt.to_pandas()`
  `>>> df`

  `>>> df = pd.read_csv('./data/co2_mm_mlo.csv', comment='#')`
  `>>> write_deltalake('./data/gen/co2_monthly_partitioned', data=df, partition_by=['year'])`
- Merging/updating
  `>>> import pyarrow as pa`
  `>>> from deltalake import DeltaTable, write_deltalake`
  `>>> data = pa.table({'id' : list(range(100))}) # Create a sample dataset`
  `>>> write_deltalake('delete-test', data)`
  `>>> dt = DeltaTable('delete-test')`
  `>>> dt.version()`
  
  `>>> dt.to_pandas().count()`
  
  `>>> dt.delete('id % 2 == 0')`
  
  `>>> dt.version() # There is a new version`
- Going beyond Pandas
  + RecordBatch
  + Table
  + DataSet
### Rust
- Reading large data
- Writing data
- Merging/updating
### Building a Lambda
