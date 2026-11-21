#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Inventory actual stable-rustdoc exports, including expanded public macros."""
import argparse
from html.parser import HTMLParser
import json
from pathlib import Path
import re
from urllib.parse import unquote, urlsplit

SOURCE_CRATES = {
    "docsis_ofdm_fec": "crates/ofdm",
    "docsis_ofdma": "crates/ofdma",
    "merlin_api": "crates/phy-api",
    "pnm_input": "crates/pnm-input",
}


class SymbolPage(HTMLParser):
    """Read item links and public declarations from compiler-generated HTML.

    Glob re-exports keep their caller namespace; ordinary public modules extend
    it. Trait implementor copies are deduplicated by the inventory consumer.
    """

    def __init__(self):
        """Initialize independent parser state without reading or writing files.

        Returns:
            None; later HTML events populate links and declarations.
        """
        super().__init__()
        self.links = []
        self.headers = []
        self.header = None
        self.source = None
        self.source_href = None
        self.code = None
        self.code_links = []

    def handle_starttag(self, tag, attrs):
        """Collect links and begin compiler declaration captures.

        Args:
            tag: HTML start-tag name supplied by HTMLParser.
            attrs: Attribute pairs for that tag.
        Returns:
            None; only this page instance changes.
        """
        attributes = dict(attrs)
        if tag == "a":
            classes = attributes.get("class", "").split()
            if set(classes) & {"struct", "enum", "mod", "fn", "trait", "type"} and attributes.get("title"):
                self.links.append(attributes)
                if self.code is not None:
                    self.code_links.append(attributes)
            if "src" in classes:
                self.source_href = attributes.get("href")
        if tag == "code":
            self.code = []
            self.code_links = []
        if ((tag == "h4" and "code-header" in attributes.get("class", "").split())
                or (tag == "pre" and "item-decl" in attributes.get("class", "").split())):
            self.header = []
            self.source = self.source_href

    def handle_data(self, data):
        """Preserve compiler text inside the current declaration/expression.

        Args:
            data: Decoded text supplied by HTMLParser.
        Returns:
            None; no caller buffers or source files change.
        """
        if self.code is not None:
            self.code.append(data)
        if self.header is not None:
            self.header.append(data)

    def handle_endtag(self, tag):
        """Finish a declaration and classify wildcard re-export expressions.

        Args:
            tag: HTML closing-tag name supplied by HTMLParser.
        Returns:
            None; completed declarations and links remain owned by this parser.
        """
        if tag == "code" and self.code is not None:
            glob = "".join(self.code).strip().endswith("::*;")
            for link in self.code_links:
                link["glob"] = glob
            self.code = None
            self.code_links = []
        if tag in ("h4", "pre") and self.header is not None:
            self.headers.append((" ".join("".join(self.header).split()), self.source))
            self.header = None


def read_page(path):
    """Read one existing compiler-generated rustdoc page.

    Args:
        path: UTF-8 page to read without modifying it.
    Returns:
        An independent SymbolPage containing links and explicit declarations.
    Raises:
        OSError or UnicodeError when the required page cannot be read.
    """
    page = SymbolPage()
    page.feed(path.read_text(encoding="utf-8"))
    return page


def source_owner(target, href):
    """Resolve a compiler source link to its project-owned source file.

    Args:
        target: Rustdoc item page containing the source link.
        href: Relative source URL, or None when rustdoc supplies no source.
    Returns:
        A repository-relative source path, or None for absent/foreign owners.
    """
    if not href:
        return None
    full = (target.parent / urlsplit(href).path).resolve()
    if "/src/" not in str(full):
        return None
    crate, relative = str(full).split("/src/", 1)[1].split("/", 1)
    if relative.endswith(".html"):
        relative = relative[:-5]
    owner = SOURCE_CRATES.get(crate)
    return f"{owner}/src/{relative}" if owner else None


def inventory(namespace, index, documentation_root):
    """Follow current public item links without scanning stale rustdoc files.

    Args:
        namespace: Public caller namespace represented by the initial index.
        index: Current compiler-generated public module page.
        documentation_root: Boundary for local documentation links.
    Returns:
        Stable item and explicit operation declarations, including macro output.
        This is compiler export discovery, never execution or parity evidence.
    Raises:
        OSError for absent required pages; malformed local links fail admission.
    """
    root = documentation_root.resolve()
    result = []
    seen = set()
    queue = [(namespace, index.resolve())]
    while queue:
        scope, path = queue.pop(0)
        for link in read_page(path).links:
            href = urlsplit(link["href"])
            target = (path.parent / unquote(href.path)).resolve()
            kind = link["class"]
            name = link["title"].split("::")[-1]
            export = scope + "::" + name
            if href.scheme or href.fragment or not target.is_relative_to(root) or target == path:
                continue
            if (export, target) in seen:
                continue
            seen.add((export, target))
            if kind == "mod":
                queue.append((scope if link.get("glob") else export, target))
                continue
            methods = {}
            for signature, source in read_page(target).headers:
                # Extract explicit public fn names: "pub fn new(...)" -> new.
                # Optional const/async/unsafe qualifiers remain in the signature.
                match = re.match(r"pub (?:(?:const|async|unsafe) )*fn (\w+)", signature)
                if match is None and kind == "trait":
                    # A public trait declaration omits pub on its required methods.
                    match = re.match(r"fn (\w+)", signature)
                if match is None:
                    continue
                operation = export if kind == "fn" else export + "::" + match[1]
                if operation in methods:
                    continue
                methods[operation] = {
                    "operation": operation,
                    "signature": signature,
                    "source_file": source_owner(target, source),
                    "rustdoc": str(target.relative_to(root)),
                }
            result.append({"export": export, "kind": kind, "methods": list(methods.values())})
    return sorted(result, key=lambda item: item["export"])


def main():
    """Write generated export facts for freshly built root/API/adapter rustdoc.

    The caller builds rustdoc with the matching Cargo feature selection first.
    Generated output belongs in ignored development storage, not user guides.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--docs", type=Path, default=Path("target/doc"))
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    roots = {
        "downstream": ("merlin::downstream", "docsis_ofdm_fec/api/index.html"),
        "upstream": ("merlin::upstream", "docsis_ofdma/api/index.html"),
        "common": ("merlin::common", "merlin_api/index.html"),
        "native-input": ("pnm_input", "pnm_input/index.html"),
    }
    data = {name: inventory(namespace, args.docs / path, args.docs)
            for name, (namespace, path) in roots.items()}
    args.output.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
    print("Rustdoc exports:", {name: sum(len(item["methods"]) for item in items)
                              for name, items in data.items()})


if __name__ == "__main__":
    main()
