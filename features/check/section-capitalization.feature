Feature: Finding section titles with varying capitalization

  Scenario: sections with different capitalization in different files
    Given the workspace contains file "lowercase.md" with content:
      """
      # One

      ### what is it

      ### how it works
      """
    And the workspace contains file "capitalized.md" with content:
      """
      # Two

      ### What is it

      ### How it works
      """
    And the workspace contains file "uppercase.md" with content:
      """
      # Three

      ### WHAT IS IT

      ### how it works
      """
    When checking the TikiBase
    Then it finds these sections with mixed capitalization:
      | how it works, How it works         |
      | what is it, What is it, WHAT IS IT |
