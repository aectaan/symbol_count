# symbol-search
Search symbols in text, counts total occurencies.
## Usage
```
symbol_count --file <file> --chars <char>...

Options:
  -f, --file <file>      File to be inspected  
  -c, --chars <char>...  Chars to be searched. Please separate them with spaces or don't separate at all. Some symbols should be shielded with backslashes, like this: "\ "  
  -h, --help             Print help  
  -V, --version          Print version  
```
## Example
```bash
symbol_count -f path/to/our/big/file.txt -chars 🍆🍑💦
```