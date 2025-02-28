``datetime::ffi``
=================

.. js:class:: ICU4XDateTime

    An ICU4X DateTime object capable of containing a date and time for any calendar.

    See the `Rust documentation for DateTime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. js:staticfunction:: create_from_iso_in_calendar(year, month, day, hour, minute, second, nanosecond, calendar)

        Creates a new :js:class:`ICU4XDateTime` representing the ISO date and time given but in a given calendar

        See the `Rust documentation for new_from_iso <https://unicode-org.github.io/icu4x-docs/doc/icu/struct.DateTime.html#method.new_from_iso>`__ for more information.


    .. js:staticfunction:: create_from_codes_in_calendar(era_code, year, month_code, day, hour, minute, second, nanosecond, calendar)

        Creates a new :js:class:`ICU4XDateTime` from the given codes, which are interpreted in the given calendar system

        See the `Rust documentation for try_new_from_codes <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.try_new_from_codes>`__ for more information.


    .. js:staticfunction:: create_from_date_and_time(date, time)

        Creates a new :js:class:`ICU4XDateTime` from an :js:class:`ICU4XDate` and :js:class:`ICU4XTime` object

        See the `Rust documentation for new <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new>`__ for more information.


    .. js:function:: date()

        Gets a copy of the date contained in this object

        See the `Rust documentation for date <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.date>`__ for more information.


    .. js:function:: time()

        Gets the time contained in this object

        See the `Rust documentation for time <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.time>`__ for more information.


    .. js:function:: to_iso()

        Converts this date to ISO

        See the `Rust documentation for to_iso <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_iso>`__ for more information.


    .. js:function:: to_calendar(calendar)

        Convert this datetime to one in a different calendar

        See the `Rust documentation for to_calendar <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_calendar>`__ for more information.


    .. js:function:: hour()

        Returns the hour in this time

        See the `Rust documentation for hour <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.hour>`__ for more information.


    .. js:function:: minute()

        Returns the minute in this time

        See the `Rust documentation for minute <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.minute>`__ for more information.


    .. js:function:: second()

        Returns the second in this time

        See the `Rust documentation for second <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.second>`__ for more information.


    .. js:function:: nanosecond()

        Returns the nanosecond in this time

        See the `Rust documentation for nanosecond <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.nanosecond>`__ for more information.


    .. js:function:: day_of_month()

        Returns the 1-indexed day in the month for this date

        See the `Rust documentation for day_of_month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.day_of_month>`__ for more information.


    .. js:function:: day_of_week()

        Returns the day in the week for this day

        See the `Rust documentation for day_of_week <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.day_of_week>`__ for more information.


    .. js:function:: week_of_month(first_weekday)

        Returns the week number in this month, 1-indexed, based on what is considered the first day of the week (often a locale preference).

        ``first_weekday`` can be obtained via ``first_weekday()`` on :js:class:`ICU4XWeekCalculator`

        See the `Rust documentation for week_of_month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.week_of_month>`__ for more information.


    .. js:function:: week_of_year(calculator)

        Returns the week number in this year, using week data

        See the `Rust documentation for week_of_year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.week_of_year>`__ for more information.


    .. js:function:: ordinal_month()

        Returns 1-indexed number of the month of this date in its year

        Note that for lunar calendars this may not lead to the same month having the same ordinal month across years; use month_code if you care about month identity.

        See the `Rust documentation for month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.month>`__ for more information.


    .. js:function:: month_code()

        Returns the month code for this date. Typically something like "M01", "M02", but can be more complicated for lunar calendars.

        See the `Rust documentation for month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.month>`__ for more information.


    .. js:function:: year_in_era()

        Returns the year number in the current era for this date

        See the `Rust documentation for year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.year>`__ for more information.


    .. js:function:: era()

        Returns the era for this date,

        See the `Rust documentation for year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.year>`__ for more information.


    .. js:function:: months_in_year()

        Returns the number of months in the year represented by this date

        See the `Rust documentation for months_in_year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.months_in_year>`__ for more information.


    .. js:function:: days_in_month()

        Returns the number of days in the month represented by this date

        See the `Rust documentation for days_in_month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.days_in_month>`__ for more information.


    .. js:function:: days_in_year()

        Returns the number of days in the year represented by this date

        See the `Rust documentation for days_in_year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.days_in_year>`__ for more information.


    .. js:function:: calendar()

        Returns the :js:class:`ICU4XCalendar` object backing this date

        See the `Rust documentation for calendar <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.calendar>`__ for more information.


.. js:class:: ICU4XIsoDateTime

    An ICU4X DateTime object capable of containing a ISO-8601 date and time.

    See the `Rust documentation for DateTime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. js:staticfunction:: create(year, month, day, hour, minute, second, nanosecond)

        Creates a new :js:class:`ICU4XIsoDateTime` from the specified date and time.

        See the `Rust documentation for try_new_iso_datetime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.try_new_iso_datetime>`__ for more information.


    .. js:staticfunction:: crate_from_date_and_time(date, time)

        Creates a new :js:class:`ICU4XIsoDateTime` from an :js:class:`ICU4XIsoDate` and :js:class:`ICU4XTime` object

        See the `Rust documentation for new <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new>`__ for more information.


    .. js:staticfunction:: create_from_minutes_since_local_unix_epoch(minutes)

        Construct from the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)

        See the `Rust documentation for from_minutes_since_local_unix_epoch <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.from_minutes_since_local_unix_epoch>`__ for more information.


    .. js:function:: date()

        Gets the date contained in this object

        See the `Rust documentation for date <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.date>`__ for more information.


    .. js:function:: time()

        Gets the time contained in this object

        See the `Rust documentation for time <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.time>`__ for more information.


    .. js:function:: to_any()

        Converts this to an :js:class:`ICU4XDateTime` capable of being mixed with dates of other calendars

        See the `Rust documentation for to_any <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_any>`__ for more information.


    .. js:function:: minutes_since_local_unix_epoch()

        Gets the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)

        See the `Rust documentation for minutes_since_local_unix_epoch <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.minutes_since_local_unix_epoch>`__ for more information.


    .. js:function:: to_calendar(calendar)

        Convert this datetime to one in a different calendar

        See the `Rust documentation for to_calendar <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_calendar>`__ for more information.


    .. js:function:: hour()

        Returns the hour in this time

        See the `Rust documentation for hour <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.hour>`__ for more information.


    .. js:function:: minute()

        Returns the minute in this time

        See the `Rust documentation for minute <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.minute>`__ for more information.


    .. js:function:: second()

        Returns the second in this time

        See the `Rust documentation for second <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.second>`__ for more information.


    .. js:function:: nanosecond()

        Returns the nanosecond in this time

        See the `Rust documentation for nanosecond <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.nanosecond>`__ for more information.


    .. js:function:: day_of_month()

        Returns the 1-indexed day in the month for this date

        See the `Rust documentation for day_of_month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.day_of_month>`__ for more information.


    .. js:function:: day_of_week()

        Returns the day in the week for this day

        See the `Rust documentation for day_of_week <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.day_of_week>`__ for more information.


    .. js:function:: week_of_month(first_weekday)

        Returns the week number in this month, 1-indexed, based on what is considered the first day of the week (often a locale preference).

        ``first_weekday`` can be obtained via ``first_weekday()`` on :js:class:`ICU4XWeekCalculator`

        See the `Rust documentation for week_of_month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.week_of_month>`__ for more information.


    .. js:function:: week_of_year(calculator)

        Returns the week number in this year, using week data

        See the `Rust documentation for week_of_year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.week_of_year>`__ for more information.


    .. js:function:: month()

        Returns 1-indexed number of the month of this date in its year

        See the `Rust documentation for month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.month>`__ for more information.


    .. js:function:: year()

        Returns the year number for this date

        See the `Rust documentation for year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.year>`__ for more information.


    .. js:function:: months_in_year()

        Returns the number of months in the year represented by this date

        See the `Rust documentation for months_in_year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.months_in_year>`__ for more information.


    .. js:function:: days_in_month()

        Returns the number of days in the month represented by this date

        See the `Rust documentation for days_in_month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.days_in_month>`__ for more information.


    .. js:function:: days_in_year()

        Returns the number of days in the year represented by this date

        See the `Rust documentation for days_in_year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.days_in_year>`__ for more information.

