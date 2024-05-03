def to_excel(input_path: str, output_path: str):
    import pandas as pd
    pd.read_csv(input_path, encoding='utf-8').to_excel(output_path, index=False)
