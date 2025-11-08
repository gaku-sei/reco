# Reco, an e-book toolbox

The repository host multiple cli and gui that allows you to edit, convert, merge, and more, e-book files (cbz, pdf, only for now, more to come).

## Tools (with supported format):

- `reco-cli convert` - cli - Convert e-books to any format (from pdf to cbz only for now)
- `reco-cli merge` - cli - Merge e-books together (cbz)
- `reco-cli pack` - cli - pack images into an e-book file (cbz)
- `reco-view-cli` - gui - A dead simple e-book reader (cbz)

## Reco Convert

Converts e-books from pdf to cbz (only now):

```bash
reco-cli convert "archive.pdf" [outdir="."] [filename="archive.cbz"]
```

## Reco Merge (cbz only for now)

This will look for all the e-books in `path` and which file name contains `something` and merge them into `output/merged_archive.cbz`:

```bash
reco-cli merge "path/**/*something*.jpg" [outdir="."] [filename="out.cbz"]
```

## Reco Pack (cbz only for now)

Takes all the images under `source` and pack them into a cbz file:

```bash
reco-cli pack "source/*.png" [outdir="."] [filename="out.cbz"] [--autosplit]
```

Options include:

- `--autosplit`: split landscape images into 2 pages

## Reco View (cbz only for now)

Read e-book files with this simple gui:

```bash
reco-view-cli "my_archive.cbz"
```
