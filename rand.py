#!/usr/bin/python3
import random
import csv
import datetime

random = [(int(random.uniform(0,640)), int(random.uniform(0,480))) for _ in range(1000)]

for i in random:
    print(f"{i[0]},{i[1]},{datetime.datetime.now()}")
