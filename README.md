# std-obs

Meant to be a library to interact with obsidian and markdown files in an efficient way.

## TODO

### Phase 1 - Functionality

- [x] Get Args working
- [ ] Verify linker works
   - Go through each file
      - If links are fine, accept them
      - If not, make changes to linker
- [ ] Linker with accepts lambda for whether to link
- [x] Write tests for each of my files
   - Test Gen Script
- [ ] Finalize Public interface
- [ ] Derive / impl common traits on all types
- [x] Fix Local vs. Absolute Path
- [ ] More Tests
   - No two strings are adjacent in a line, should be linked

### Phase 2 - Optimization

- [ ] Rayon Feature Flag
- [ ] Serde save & load
   - sqlite?

### Phase 3 - Organization

- [ ] Restructure Structs & Enums
- [ ] Add Code Comments to all methods
- [ ] Get Lint on save working & clippy style guide?
   - Bacon too?

### Phase 4 - Deployment

- [ ] Github automations
- [ ] Usages
   - Exe
      - x86
      - Arm
      - WASM
   - DLL
   - Crate