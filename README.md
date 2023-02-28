# Unicode Filename Normalizer

Unicode normalize filenames in folder to form one of [NFC, NFD, NFKC, NFKD]
It works for unicode normalize issues in samba or nfs on macOS，see [detail](https://www.reddit.com/r/DataHoarder/comments/ffqnra/making_smbnfsafp_shares_unicode_normalisation/).

## Usage
```
Usage: unicode-normalizer [OPTIONS] --to-form <TO_FORM> [PATH]...

Arguments:
  [PATH]...  Path to be convert

Options:
  -t, --to-form <TO_FORM>  Normalize form [possible values: NFC, NFD, NFKC, NFKD]
  -l, --log <LOG>          Log file path [default: convert.log]
      --dry-run            Dry run convert
  -h, --help               Print help
```

macOS unicode normalize issues work around, run this on your NFS/Samba server device.
- NFS: --to-form NFD
- Samba: --to-form NFC

## Goal
Learn rust by writing real project.

Some nice rust projects worth learning about.
- https://github.com/jmacdonald/amp
