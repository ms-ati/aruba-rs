Feature: Help text

  Scenario: `--help` shows usage documentation
    When I run `aruba_clap_hello_world --help`
    Then the exit status code should be 0
     And the stdout contains exactly:
         """
         Simple program to greet a person

         Usage: aruba_clap_hello_world [OPTIONS] --name <NAME>

         Options:
           -n, --name <NAME>    Name of the person to greet
           -c, --count <COUNT>  Number of times to greet [default: 1]
           -h, --help           Print help information
           -V, --version        Print version information
         """

  Scenario: `--version` shows application version
    When I run `aruba_clap_hello_world --version`
    Then the exit status code should be 0
     And the stdout contains exactly: "aruba_clap_hello_world 0.1.0"