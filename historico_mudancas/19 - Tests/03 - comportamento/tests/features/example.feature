Feature: Example feature

  Scenario Outline: An example scenario
    Given foo is 0
      | sample       | val1   | val2   |
      | longer value | <val1> | <val2> |
    When foo is 0

    Examples:
      | val1 | val2 |
      | 1    | 4    |
      | 2    | 5    |
      | 3    | 6    |

  Scenario: An example sync scenario
    Given foo is sync 0

  Scenario: Steps returning result
    When I write "abc" to 'myfile.txt'
    Then the file 'myfile.txt' should contain "abc"

  Scenario: Steps capturing into slices
    When I write "abc" to 'myfile.txt'
    Then "myfile.txt" contains 'abc'

