# TESTING
when writing unit or integration test be sure to include the tests into the existing files and follow the standard convention already established with the chat-cli crate

- all new code must have an associated test.
- all code changed/modified/updated/refactored must have associated tests
- test coverage of 80% is minimum to be considered "good coverage"
- test coverage of 90% is minimum to be considered "robust"

## Unit tests
be sure to use mocs for any external dependencies, APIs, or file read/write operations. tests that interact with other crates are not unit tests, they are integration tests.

## Integration tests
be sure to use mocs for dependencies outside of the project and that are not in a crate under the crates dir. This includes AWS API calls and file system read/write
