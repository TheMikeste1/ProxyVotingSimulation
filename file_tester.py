import os.path

import pandas as pd

file_directory = "data"
filename = "2023-01-15_20-39-25.arrow"

df = pd.read_feather(os.path.join(file_directory, filename))

pd.set_option('display.max_columns', None)
pd.set_option('display.max_rows', None)

print("Successfully read file!")
df.info()

