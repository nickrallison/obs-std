# std-obs

Meant to be a library to interact with obsidian and markdown files in an efficient way.

## TODO

Lint code on Save

Reorganization
 - Move all Struct

Serialize & Deserialize vs. Parsing 
 - Profile difference
 - Serialization based on metadata
 - Public Types & Functions Specification

Env Vars
 - Logging
 - Link to self
 - Link to other via lambda of the two files

Features
 - Unlinking
 - Linking Individual Files
 - Link Must contain same tag
 - Link with condition
 - Derive common traits on public types

Color Eyre

Add Link Type
 - String
 - Target Path

Make sure local vs absolute done correctly

Add assert to absolute path that it is inside vault path

Add Default Traits to types

Tests
 - No two Node::Strings are adjacent (Without a newline)

## Notes

Public Structs
 - Vault
 - MDFile
 - Link

Public Functions
 - Vault
 - File
   - new
   - from_path
   - from_str
 
   