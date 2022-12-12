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
