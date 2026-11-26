/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml/shared/presentation"
import "fixtures"

TestCase {
    id: testCase
    name: "PresentationOwner"
    when: windowShown
    PresentationBudget { id: budget }
    QtObject { id: fixtureTheme; property var presentationBackendFactory: budget.createAdapter }
    Component { id: ownerComponent; PresentationOwner { theme: fixtureTheme } }

    function owner() {
        const result = createTemporaryObject(ownerComponent, testCase)
        verify(result !== null)
        return result
    }
    function init() {
        compare(budget.bytes, 0)
        compare(budget.entries, 0)
        compare(budget.views, 0)
        compare(budget.open, 0)
        budget.maximumBytes = 24
        budget.maximumEntries = 12
        budget.maximumViews = 3
        budget.maximumOpen = 1
        budget.failCommit = false
    }
    function test_exact_utf16_replacement_overlap_and_rejected_candidate() {
        const value = owner()
        budget.maximumBytes = 6
        verify(value.replace("a"))
        compare(budget.bytes, 2)
        verify(value.replace("😀")) // Four UTF-16 bytes; old plus candidate exactly six.
        compare(value.value, "😀")
        compare(budget.bytes, 4)
        verify(!value.replace("bb"))
        compare(value.value, "😀")
        compare(budget.bytes, 4)
        verify(value.rejected)
        verify(value.message.length > 0)
        verify(value.clear())
        compare(value.value, null)
        compare(budget.bytes, 0)
    }
    function test_entries_and_unsupported_values_preserve_old_presentation() {
        const value = owner()
        budget.maximumEntries = 5
        verify(value.replace([null, null]))
        verify(value.replace([null, null, null]))
        compare(budget.entries, 3)
        verify(!value.replace([null, null, null]))
        compare(value.value.length, 3)
        verify(!value.replace({invalid: NaN}))
        compare(value.value.length, 3)
        compare(budget.entries, 3)
    }
    function test_hidden_data_and_open_slot_have_independent_lifetimes() {
        const first = owner()
        const second = owner()
        verify(first.replace("kept"))
        verify(first.beginOpen())
        verify(first.beginOpen())
        compare(budget.open, 1)
        verify(!second.beginOpen())
        compare(budget.open, 1)
        first.endOpen()
        compare(budget.open, 0)
        compare(first.value, "kept")
        compare(budget.bytes, 8)
        verify(second.beginOpen())
        compare(budget.open, 1)
        verify(first.clear())
        compare(budget.bytes, 0)
        compare(budget.open, 1)
        second.endOpen()
    }
    function test_failed_commit_rolls_back_value_and_only_pending_coverage() {
        const value = owner()
        verify(value.replace("old"))
        budget.failCommit = true
        verify(!value.replace("new"))
        compare(value.value, "old")
        compare(budget.bytes, 6)
        compare(budget.entries, 0)
    }
    function test_reentrant_clear_and_replace_cannot_destroy_transaction_coverage() {
        const value = owner()
        verify(value.replace("old"))
        let nestedClear = true
        let nestedReplace = true
        let callbacks = 0
        value.valueChanged.connect(function() {
            callbacks++
            nestedClear = value.clear()
            nestedReplace = value.replace("bad")
        })
        verify(value.replace("new"))
        compare(callbacks, 1)
        verify(!nestedClear)
        verify(!nestedReplace)
        compare(value.value, "new")
        compare(budget.bytes, 6)
        compare(budget.entries, 0)
    }
    Component { id: textFieldComponent; TextField { maximumLength: 3 } }
    function test_text_capacity_is_finite_and_survives_data_clear() {
        const value = owner()
        const field = createTemporaryObject(textFieldComponent, testCase)
        verify(field !== null)
        budget.maximumBytes = 8
        verify(value.reserveTextFields([field]))
        verify(value.textAdmitted)
        compare(budget.bytes, 6)
        field.text = "abcdef"
        compare(field.text, "abc")
        field.text = "😀ab"
        verify(field.text.length <= field.maximumLength)
        verify(value.replace("x"))
        compare(budget.bytes, 8)
        verify(value.clear())
        compare(value.value, null)
        compare(budget.bytes, 6)
        verify(value.textAdmitted)
    }
    function test_text_capacity_rejection_and_retry_preserve_current_value() {
        const value = owner()
        const field = createTemporaryObject(textFieldComponent, testCase)
        verify(field !== null)
        budget.maximumBytes = 5
        verify(!value.reserveTextFields([field]))
        verify(!value.textAdmitted)
        verify(value.rejected)
        compare(budget.bytes, 0)
        compare(budget.views, 0)
        budget.maximumBytes = 6
        verify(value.reserveTextFields([field]))
        compare(budget.bytes, 6)
        verify(value.reserveTextFields([field]))
        compare(budget.bytes, 6)
        compare(budget.views, 1)
    }
    function test_retained_view_population_rejects_and_releases_after_clear() {
        const first = owner()
        const second = owner()
        budget.maximumViews = 1
        verify(first.replace(""))
        compare(budget.views, 1)
        verify(!second.replace(""))
        compare(second.value, null)
        compare(budget.views, 1)
        verify(first.clear())
        compare(budget.views, 0)
        verify(second.replace(""))
        compare(budget.views, 1)
    }
    function test_missing_factory_fails_closed() {
        const value = createTemporaryObject(ownerComponent, testCase,
            {theme: {presentationBackendFactory: null}})
        verify(value !== null)
        verify(!value.replace("unadmitted"))
        verify(!value.beginOpen())
        compare(value.value, null)
        compare(budget.bytes, 0)
        compare(budget.views, 0)
    }
}
