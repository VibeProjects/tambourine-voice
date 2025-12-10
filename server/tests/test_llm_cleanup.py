"""Tests for LLM cleanup prompt combination logic."""

from processors.llm_cleanup import (
    ADVANCED_PROMPT_DEFAULT,
    DICTIONARY_PROMPT_DEFAULT,
    MAIN_PROMPT_DEFAULT,
    combine_prompt_sections,
)


class TestCombinePromptSections:
    """Tests for combine_prompt_sections() function."""

    def test_all_disabled_returns_empty_string(self) -> None:
        """When all sections are disabled, returns empty string."""
        result = combine_prompt_sections(
            main_enabled=False,
            main_content=None,
            advanced_enabled=False,
            advanced_content=None,
            dictionary_enabled=False,
            dictionary_content=None,
        )
        assert result == ""

    def test_main_enabled_uses_default(self) -> None:
        """When main enabled with no custom content, uses default."""
        result = combine_prompt_sections(
            main_enabled=True,
            main_content=None,
            advanced_enabled=False,
            advanced_content=None,
            dictionary_enabled=False,
            dictionary_content=None,
        )
        assert result == MAIN_PROMPT_DEFAULT

    def test_main_enabled_with_custom_content(self) -> None:
        """When main enabled with custom content, uses custom content."""
        custom = "My custom main prompt"
        result = combine_prompt_sections(
            main_enabled=True,
            main_content=custom,
            advanced_enabled=False,
            advanced_content=None,
            dictionary_enabled=False,
            dictionary_content=None,
        )
        assert result == custom
        assert MAIN_PROMPT_DEFAULT not in result

    def test_advanced_enabled_uses_default(self) -> None:
        """When advanced enabled with no custom content, uses default."""
        result = combine_prompt_sections(
            main_enabled=False,
            main_content=None,
            advanced_enabled=True,
            advanced_content=None,
            dictionary_enabled=False,
            dictionary_content=None,
        )
        assert result == ADVANCED_PROMPT_DEFAULT

    def test_dictionary_enabled_uses_default(self) -> None:
        """When dictionary enabled with no custom content, uses default."""
        result = combine_prompt_sections(
            main_enabled=False,
            main_content=None,
            advanced_enabled=False,
            advanced_content=None,
            dictionary_enabled=True,
            dictionary_content=None,
        )
        assert result == DICTIONARY_PROMPT_DEFAULT

    def test_multiple_sections_joined_with_double_newline(self) -> None:
        """Multiple enabled sections are joined with double newlines."""
        result = combine_prompt_sections(
            main_enabled=True,
            main_content="Main",
            advanced_enabled=True,
            advanced_content="Advanced",
            dictionary_enabled=True,
            dictionary_content="Dictionary",
        )
        assert result == "Main\n\nAdvanced\n\nDictionary"

    def test_order_is_main_advanced_dictionary(self) -> None:
        """Sections appear in order: main, advanced, dictionary."""
        result = combine_prompt_sections(
            main_enabled=True,
            main_content="AAA",
            advanced_enabled=True,
            advanced_content="BBB",
            dictionary_enabled=True,
            dictionary_content="CCC",
        )
        parts = result.split("\n\n")
        assert parts == ["AAA", "BBB", "CCC"]

    def test_skipped_sections_do_not_leave_gaps(self) -> None:
        """Disabled sections don't leave extra newlines."""
        result = combine_prompt_sections(
            main_enabled=True,
            main_content="Main",
            advanced_enabled=False,
            advanced_content=None,
            dictionary_enabled=True,
            dictionary_content="Dictionary",
        )
        assert result == "Main\n\nDictionary"
        # No triple newlines from skipped section
        assert "\n\n\n" not in result

    def test_empty_string_content_treated_as_falsy(self) -> None:
        """Empty string content should use default (empty string is falsy)."""
        result = combine_prompt_sections(
            main_enabled=True,
            main_content="",
            advanced_enabled=False,
            advanced_content=None,
            dictionary_enabled=False,
            dictionary_content=None,
        )
        assert result == MAIN_PROMPT_DEFAULT

    def test_default_combination_main_and_advanced(self) -> None:
        """Default app behavior: main + advanced enabled, dictionary disabled."""
        result = combine_prompt_sections(
            main_enabled=True,
            main_content=None,
            advanced_enabled=True,
            advanced_content=None,
            dictionary_enabled=False,
            dictionary_content=None,
        )
        assert MAIN_PROMPT_DEFAULT in result
        assert ADVANCED_PROMPT_DEFAULT in result
        assert DICTIONARY_PROMPT_DEFAULT not in result
