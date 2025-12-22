# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Essential Commands

### Build and Test Programs
```bash
# Build all Solana programs
pnpm programs:build

# Run program tests
pnpm programs:test

# Run program tests with debug logs
pnpm programs:debug

# Clean built programs
pnpm programs:clean
```

### Client Testing
```bash
# Test Rust client
pnpm clients:rust:test

# Test JavaScript client  
pnpm clients:js:test
```

### Code Generation
```bash
# Generate IDLs and clients (run after program changes)
pnpm generate

# Generate IDLs only
pnpm generate:idls

# Generate clients only
pnpm generate:clients
```

### Local Validator
```bash
# Start local validator
pnpm validator

# Start with debug logs
pnpm validator:debug

# View logs
pnpm validator:logs

# Stop validator
pnpm validator:stop
```

### Formatting and Linting
```bash
# Format and fix code style
pnpm lint:fix

# Check code style
pnpm lint
```

## Architecture

### Program Structure
The main Solana program is located in `programs/mpl-project-name/` with standard Rust program organization:
- `src/lib.rs` - Main library file declaring the program ID
- `src/entrypoint.rs` - Program entrypoint
- `src/instruction.rs` - Instruction definitions with Shank macros for IDL generation
- `src/processor.rs` - Instruction processing logic
- `src/state.rs` - Account state definitions
- `src/error.rs` - Custom error types

### Client Generation Pipeline
1. **Shank** (`configs/shank.cjs`) - Extracts IDL from Rust program annotations, outputs to `idls/`
2. **Kinobi** (`configs/kinobi.cjs`) - Generates TypeScript and Rust clients from IDL:
   - JavaScript client: `clients/js/src/generated/`
   - Rust client: `clients/rust/src/generated/`

### Key Configuration Files
- `configs/validator.cjs` - Amman validator configuration with program deployments
- `configs/kinobi.cjs` - Client generation configuration including PDA seeds and account discriminators
- `.github/.env` - CI/CD environment variables (Rust/Solana versions, program names)

### Account Model
The template uses discriminated accounts with a `Key` enum field for account type identification. PDAs are configured in Kinobi with seed definitions for deterministic address derivation.

## Development Workflow

1. Make changes to the Rust program in `programs/mpl-project-name/`
2. Run `pnpm programs:build` to build the program
3. Run `pnpm programs:test` to test your changes
4. Run `pnpm generate` to regenerate IDLs and clients
5. Test clients with `pnpm clients:js:test` and `pnpm clients:rust:test`
6. Use `pnpm validator` for integration testing with a local validator

## Important Notes

- The project uses pnpm as the package manager (v8.9.0)
- Solana version: 2.2.1, Rust version: 1.83.0
- Programs are built to `programs/.bin/` for deployment
- External programs (Token Metadata, SPL Noop) are fetched during build for local validator
- Always regenerate clients after program changes using `pnpm generate`