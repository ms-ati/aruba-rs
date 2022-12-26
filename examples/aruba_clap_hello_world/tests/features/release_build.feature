Feature: [Release build] Greet a person

  Background:
    Given I successfully run `cargo build --release` in the current directory
    And I prepend `${PROJECT_ROOT}/target/release` to PATH

  Scenario: [Release build] Greets name
    When I run `aruba_clap_hello_world --name Aruba`
    Then the exit status code should be 0
    And the stdout contains exactly: "Hello Aruba!"

  Scenario: [Release build] Finds release build first in PATH
    When I run `which aruba_clap_hello_world`
    Then the stdout contains: "target/release/aruba_clap_hello_world"
