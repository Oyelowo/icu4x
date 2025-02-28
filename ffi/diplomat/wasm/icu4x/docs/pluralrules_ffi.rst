``pluralrules::ffi``
====================

.. js:class:: ICU4XPluralCategories

    FFI version of ``PluralRules::categories()`` data.


    .. js:attribute:: zero

    .. js:attribute:: one

    .. js:attribute:: two

    .. js:attribute:: few

    .. js:attribute:: many

    .. js:attribute:: other

.. js:class:: ICU4XPluralCategory

    FFI version of ``PluralCategory``.

    See the `Rust documentation for PluralCategory <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/enum.PluralCategory.html>`__ for more information.


    .. js:staticfunction:: get_for_cldr_string(s)

        Construct from a string in the format `specified in TR35 <https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>`__

        See the `Rust documentation for get_for_cldr_string <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/enum.PluralCategory.html#method.get_for_cldr_string>`__ for more information.

        See the `Rust documentation for get_for_cldr_bytes <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/enum.PluralCategory.html#method.get_for_cldr_bytes>`__ for more information.


.. js:class:: ICU4XPluralOperands

    FFI version of ``PluralOperands``.

    See the `Rust documentation for PluralOperands <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralOperands.html>`__ for more information.


    .. js:staticfunction:: create_from_string(s)

        Construct for a given string representing a number

        See the `Rust documentation for from_str <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralOperands.html#method.from_str>`__ for more information.


.. js:class:: ICU4XPluralRules

    FFI version of ``PluralRules``.

    See the `Rust documentation for PluralRules <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html>`__ for more information.


    .. js:staticfunction:: create_cardinal(provider, locale)

        Construct an :js:class:`ICU4XPluralRules` for the given locale, for cardinal numbers

        See the `Rust documentation for try_new_cardinal_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html#method.try_new_cardinal_unstable>`__ for more information.


    .. js:staticfunction:: create_ordinal(provider, locale)

        Construct an :js:class:`ICU4XPluralRules` for the given locale, for ordinal numbers

        See the `Rust documentation for try_new_ordinal_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html#method.try_new_ordinal_unstable>`__ for more information.


    .. js:function:: category_for(op)

        Get the category for a given number represented as operands

        See the `Rust documentation for category_for <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html#method.category_for>`__ for more information.


    .. js:function:: categories()

        Get all of the categories needed in the current locale

        See the `Rust documentation for categories <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html#method.categories>`__ for more information.

