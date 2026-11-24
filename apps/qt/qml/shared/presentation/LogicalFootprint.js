/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
.pragma library

// Encoding quantity, independent of application admission and scientific limits.
const UTF16_BYTES_PER_CODE_UNIT = 2;
const INVALID_LIMITS = "presentation.invalid_limits";
const BYTE_LIMIT = "presentation.utf16_bytes";
const ENTRY_LIMIT = "presentation.entries";
const DEPTH_LIMIT = "presentation.depth";
const CYCLIC_VALUE = "presentation.cycle";
const UNSUPPORTED_VALUE = "presentation.unsupported";

/**
 * Measure one explicitly retained logical presentation value without mutation.
 * @param value Plain dense arrays/data objects and finite JSON scalar values.
 * @param limits Nonnegative safe-integer max_utf16_bytes, max_entries, max_depth.
 * @returns Success with UTF-16 bytes, separate array/object entries and their sum,
 * or {ok:false,error:<static code>} without partial counts or input diagnostics.
 * Root depth is zero; keys and string values count actual UTF-16 code units.
 * Repeated noncyclic references count per occurrence, not physical allocation.
 * Callers handle parent/plot aliases and independent derived complements. This
 * operation does not reserve resources, choose limits or retain/cache inputs.
 */
function measure(value, limits) {
    function failure(code) {
        return { ok: false, error: code };
    }

    function validLimit(name) {
        const descriptor = Object.getOwnPropertyDescriptor(limits, name);
        return descriptor && Object.prototype.hasOwnProperty.call(descriptor, "value")
            && Number.isSafeInteger(descriptor.value) && descriptor.value >= 0;
    }

    try {
        if (!limits || typeof limits !== "object" || Array.isArray(limits)
                || !validLimit("max_utf16_bytes") || !validLimit("max_entries")
                || !validLimit("max_depth")) {
            return failure(INVALID_LIMITS);
        }
        const maximumBytes = Object.getOwnPropertyDescriptor(limits, "max_utf16_bytes").value;
        const maximumEntries = Object.getOwnPropertyDescriptor(limits, "max_entries").value;
        const maximumDepth = Object.getOwnPropertyDescriptor(limits, "max_depth").value;
        let utf16Bytes = 0;
        let arrayEntries = 0;
        let objectEntries = 0;
        let entries = 0;
        const ancestors = new Set();
        const pending = [{ value: value, depth: 0 }];

        function addString(text) {
            if (text.length > Math.floor((maximumBytes - utf16Bytes) / UTF16_BYTES_PER_CODE_UNIT)) {
                return false;
            }
            utf16Bytes += text.length * UTF16_BYTES_PER_CODE_UNIT;
            return true;
        }

        while (pending.length > 0) {
            const current = pending.pop();
            if (current.leave) {
                ancestors.delete(current.value);
                continue;
            }
            if (current.depth > maximumDepth) {
                return failure(DEPTH_LIMIT);
            }
            const item = current.value;
            const type = typeof item;
            if (type === "string") {
                if (!addString(item)) {
                    return failure(BYTE_LIMIT);
                }
                continue;
            }
            if (item === null || type === "boolean" || (type === "number" && Number.isFinite(item))) {
                continue;
            }
            if (type !== "object") {
                return failure(UNSUPPORTED_VALUE);
            }
            if (ancestors.has(item)) {
                return failure(CYCLIC_VALUE);
            }

            const array = Array.isArray(item);
            const prototype = Object.getPrototypeOf(item);
            if ((array && prototype !== Array.prototype)
                    || (!array && prototype !== Object.prototype && prototype !== null)
                    || Object.getOwnPropertySymbols(item).length > 0) {
                return failure(UNSUPPORTED_VALUE);
            }
            if (array && item.length > maximumEntries - entries) {
                return failure(ENTRY_LIMIT);
            }
            const names = Object.getOwnPropertyNames(item);
            const count = array ? item.length : names.length;
            if (!Number.isSafeInteger(count) || count > maximumEntries - entries) {
                return failure(ENTRY_LIMIT);
            }
            if (array && (names.length !== count + 1 || names[count] !== "length")) {
                return failure(UNSUPPORTED_VALUE);
            }
            if (count > 0 && current.depth >= maximumDepth) {
                return failure(DEPTH_LIMIT);
            }
            entries += count;
            if (array) {
                arrayEntries += count;
            } else {
                objectEntries += count;
            }

            ancestors.add(item);
            pending.push({ value: item, leave: true });
            for (let index = count - 1; index >= 0; index--) {
                const name = names[index];
                if (array && name !== String(index)) {
                    return failure(UNSUPPORTED_VALUE);
                }
                if (!array && !addString(name)) {
                    return failure(BYTE_LIMIT);
                }
                const descriptor = Object.getOwnPropertyDescriptor(item, name);
                if (!descriptor || !Object.prototype.hasOwnProperty.call(descriptor, "value")) {
                    return failure(UNSUPPORTED_VALUE);
                }
                pending.push({ value: descriptor.value, depth: current.depth + 1 });
            }
        }
        return {
            ok: true,
            utf16_bytes: utf16Bytes,
            array_entries: arrayEntries,
            object_entries: objectEntries,
            entries: entries
        };
    } catch (error) {

        // Unsupported introspection never exports exception/input text.
        return failure(UNSUPPORTED_VALUE);
    }
}

/**
 * Bound the existing editable-text complement without constructing dummy text.
 * @param lengths Plain array of actual Qt maximumLength code-unit limits.
 * @param maximumBytes Inclusive logical UTF-16 policy, independent of DTO limits.
 * @returns A conservative byte bound or a static failure; no input is retained.
 * The caller admits this bound before enabling/populating those controls.
 */
function textCapacity(lengths, maximumBytes) {
    if (!Array.isArray(lengths) || !Number.isSafeInteger(maximumBytes) || maximumBytes < 0) {
        return {ok: false, error: INVALID_LIMITS};
    }
    let bytes = 0;
    for (const length of lengths) {
        if (!Number.isSafeInteger(length) || length < 0) {
            return {ok: false, error: UNSUPPORTED_VALUE};
        }
        if (length > Math.floor((maximumBytes - bytes) / UTF16_BYTES_PER_CODE_UNIT)) {
            return {ok: false, error: BYTE_LIMIT};
        }
        bytes += length * UTF16_BYTES_PER_CODE_UNIT;
    }
    return {ok: true, utf16_bytes: bytes};
}
