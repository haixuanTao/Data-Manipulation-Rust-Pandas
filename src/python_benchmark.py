import pandas as pd

df = pd.read_csv(
    "/home/peter/Documents/TEST/RUST/terrorism/src/globalterrorismdb_0718dist.csv"
)

# 1. Filtering
df = df[df.country_txt == "United States"]
df.to_csv("python_output.csv")

# 2. Group by
df = df.groupby(by="country_txt", as_index=False).agg(
    {"nkill": "sum", "individual": "mean", "eventid": "count"}
)
df.to_csv("python_output_groupby.csv")

# 3. Mutation
df["computed_file"] = df["nkill"].map(lambda x: (x - 10) / 2 + x ** 2 / 3)
df.to_csv("python_output_map.csv")

# 4. Join
df_country = pd.read_csv(
    "/home/peter/Documents/TEST/RUST/terrorism/src/WDICountry.csv"
)

df_merge = pd.merge(
    df, df_country, left_on="country_txt", right_on="Short_Name"
)
df_merge.to_csv("python_output_merge.csv")