# examples/aruba_hello_world

A test command-line application based on the [Clap](https://github.com/clap-rs/clap/blob/master/README.md) 4.0.x
[example](https://docs.rs/clap/4.0.17/clap/index.html).

```gherkin
Feature: Greet a person

  Scenario: Greets name count times
    When I run `aruba_clap_hello_world --name Aruba`
    Then the exit status code should be 0
     And the stdout contains exactly: "Hello Aruba!"

    When I run `aruba_clap_hello_world --name Aruba --count 3`
    Then the exit status code should be 0
     And the stdout contains exactly:
         """
         Hello Aruba!
         Hello Aruba!
         Hello Aruba!
         """
```
