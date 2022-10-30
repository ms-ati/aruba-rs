Feature: Help text

  Scenario: If we pass the `--help` flag, we see usage documentation
    When I run `aruba_hello_world --help`
    Then the exit status code should be 0
     And the stdout contains: "Usage: "
