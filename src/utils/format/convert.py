import pandas as pd

def read_and_save_csv(input_path, output_path):
    df = pd.read_csv(input_path, encoding='utf-8')
    df.to_excel(output_path, index=False)
