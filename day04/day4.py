import pandas as pd

file = pd.read_csv("./data/day4.csv", header=None)
file.columns = ["first", "second"]
file[["f_s", "f_e"]] = pd.DataFrame(
    file["first"].str.split("-").tolist(), index=file.index
)
file[["s_s", "s_e"]] = pd.DataFrame(
    file["second"].str.split("-").tolist(), index=file.index
)

file[["f_s", "f_e", "s_s", "s_e"]] = file[["f_s", "f_e", "s_s", "s_e"]].astype(int)
filt = (file["f_s"] <= file["s_s"]) * (file["f_e"] >= file["s_e"]) | (
    file["f_s"] >= file["s_s"]
) * (file["f_e"] <= file["s_e"])
file["full"] = filt


print(sum(file["full"]))


filt2 = (file["f_e"] < file["s_s"]) | (file["f_s"] > file["s_e"])
file["no"] = filt2

print(sum(~file["no"]))
