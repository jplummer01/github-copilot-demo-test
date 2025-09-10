# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

## Testing

This project includes comprehensive unit tests to ensure code quality and functionality. Run tests using:

```bash
cargo test
```

### Test Coverage

The test suite includes:

- **Data Structure Tests**: Validates JSON deserialization of API responses
- **Error Handling Tests**: Ensures robust error handling for invalid data
- **Validation Tests**: Tests helper functions and URL validation logic
- **Component Tests**: Validates asset constants and compilation

### Test Philosophy

Tests focus on:
- Pure functions and data transformations
- Error conditions and edge cases  
- API contract validation
- Code compilation and structure

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

