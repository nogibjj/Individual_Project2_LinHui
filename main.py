import pandas as pd
import time

start_time = time.time()

# 1. Load
data = pd.read_csv('AAPL.csv')

# 2. Calculate the average closing price
average_close = data['Close'].mean()


print(f"Average Closing Price: {average_close}")

end_time = time.time()
print(f"Execution time: {end_time - start_time} seconds")
