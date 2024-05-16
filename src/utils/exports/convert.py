def arrow_to_excel(
    arrow_batch,
    dest_path: str,
):
    import polars as pl
    batch = pl.from_arrow(arrow_batch)
    batch.write_excel(dest_path)
