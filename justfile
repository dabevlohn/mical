# List all recipes
list:
	just -l

# Convert CSV to Parquet
topar:
	qsv sqlp -d '|' mytasks_2024.csv 'select * from mytasks_2024' --format parquet --output data.parquet

# Select columns
selcol:
	qsv select -d '|' '!/^(Пользовательское)|(Внешняя)|(Наблюдатели)|(Сделать)|(Входящая)/' mytasks_2024.csv 

