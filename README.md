# std-obs

Meant to be a library to interact with obsidian and markdown files in an efficient way.

## Directions

### Run Tests

You have to generate the tests first before running them they are dynamically generated so they are not commited to git.
```bash
python3 scripts/testgen.py test_vaults/reference_linked_noself test_vaults/reference_clean src/test_suite.rs
cargo test 
```

## Run Binary

The binary has a few actions to run it with. It supports the following:
- `none` - Does nothing
- `link` - Links all the files in the vault
- `unlink` - Unlinks all the files in the vault
- `alias-tree` - Prints the alias tree

It supports the following options as well:
- `preview` - Does not write to disk, just prints the changes
- `safe` - Writes to disk, after prompting the user to confirm the changes
- `force` - Writes to disk, without prompting the user to confirm the changes
- `time` - Does not write to disk, does not print changes, just prints the time it took to run the action

It also needs a target path to the vault to run on.

```bash
# Run the binary with the link action, the safe option, and the target path of test_vaults/reference_linked_noself
cargo run --release -- --action link --option safe --target-path test_vaults/reference_linked_noself

# Run the binary with the alias-tree action, the option is silently ignored here, and the target path of test_vaults/reference_linked_noself
cargo run --release -- --action alias-tree --option preview --target-path test_vaults/reference_linked_noself
```

## TODO

### Phase 1 - Functionality

- [ ] Linker with accepts lambda for whether to link
- [ ] More Tests
   - No two strings are adjacent in a line, should be linked
- [ ] Fix Multiple Notes with the same alias

### Phase 2 - Optimization

- [ ] Rayon Feature Flag
- [ ] Serde save & load
   - sqlite?
- [ ] Better Profiling
- [ ] Optimize mdparser
  - Give File Bufreader so it can read as it parses

### Phase 3 - Organization

- [ ] Restructure Structs & Enums
- [ ] Add Code Comments to all methods
- [ ] Get Lint on save working & clippy style guide?
   - Bacon too?
- [ ] Add Documentation

### Phase 4 - Deployment

- [ ] Github automations
- [ ] Usages
   - Exe
      - x86
      - Arm
      - WASM
   - DLL
   - Crate

### Phase 5+ - Beyond

Other Features
- Tag Manipulation
- Tag Splitting
- Find Links that don't belong
  - Embeddings Matching?
- Individual Linking Rules Per File
  - eg. incoming must match tag
  - eg. outgoing must match tag or this other tag...
  - eg. incoming must be at least this distance in embedding space
  - User Customizable?
- Check for broken links

## Documentation

### Types

#### cli.rs

Action
Option
CLI

The logic for the commandline interface of the compiled executable lives here.

#### linking.rs

Link
 - alias: String
 - path: PathBuf
LinkerOptions

The Link Type contains the information for a link between two files.
The LinkerOptions Type contains the options for the linker.

#### mdfile.rs

MDFile
- path: PathBuf
- title: String
- aliases: Vec<String>
- ast: AST
- last_modified: Option<std::time::SystemTime>

The MDFile Type contains the information for a markdown file.
It is the main type that the library is built around.

#### parse.rs

AST
- blocks: Vec<Block>
- line_sep: NewlineType
NewlineType
Block
- Enum:
  - YAML
  - CodeBlock
    - Vec<String>
  - LayexBlock
    - Vec<String>
  - BlockQuote
    - Vec<Block>
  - TextBlock
    - Vec<Line>
Line
- Enum
  - Heading
    - Vec<Node>
  - BulletPoint
    - Vec<Node>
  - ListItem
    - Vec<Node>
  - String
    - Vec<Node>
  - LineBar
Node
  - Enum
    - InlineCode
    - InlineBlockLatex
    - InlineLatex
    - FormattedMarkdownLink
    - MarkdownLink
    - FormattedWebLink
    - WebLink
    - BoldItalic
      - Vec<Node>
    - Bold
      - Vec<Node>
    - Italic
      - Vec<Node>
    - String
      - String

BlockParseState
NodeParseState

#### stringtree.rs

StringTree<T>
- end: Option<Vec<T>>,
- children: HashMap<String, StringTree<T>>,

This is a recursive tree structure that is used to efficiently store the aliases of each note. 
Allow for searching in closer to linear time than a brute force regex search.

#### vault.rs

Vault

The Vault Type contains the information for a vault of markdown files.
This has the logic for searching and linking between files. 
 - i.e. determining which files should be linked together. The linking itself is done in the linking module.