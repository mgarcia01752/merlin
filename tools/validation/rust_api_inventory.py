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
    "merlin": "crates/merlin",
    "merlin_api": "crates/phy-api",
    "pnm_input": "crates/pnm-input",
}
API_NAMESPACES = {
    'docsis_ofdm_fec::api': 'merlin::downstream',
    'docsis_ofdma::api': 'merlin::upstream',
    'merlin_api': 'merlin::common',
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
            declaration = " ".join("".join(self.code).split())
            glob = declaration.endswith("::*;")
            renamed = re.fullmatch(r"pub use [\w:]+ as (\w+);", declaration)
            if renamed and len(self.code_links) == 1:
                self.code_links[0]["export_name"] = renamed[1]
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
            name = link.get("export_name", link["title"].split("::")[-1])
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


def compiler_primitive_aliases(method, documentation_root):
    """Read primitive type-alias declarations linked by the canonical compiler page.

    Args:
        method: Canonical rustdoc method with its actual item page.
        documentation_root: Fresh compiler output boundary, or None.
    Returns:
        A map of exact linked alias names to their compiler-declared primitives.
        Unknown/compound aliases remain unresolved; names alone are never evidence.
    """
    if documentation_root is None or 'rustdoc' not in method:
        return {}
    root = Path(documentation_root).resolve()
    page = root / method['rustdoc']
    aliases = {}
    for link in read_page(page).links:
        if link['class'] != 'type':
            continue
        href = urlsplit(link['href'])
        target = (page.parent / unquote(href.path)).resolve()
        if href.scheme or not target.is_relative_to(root) or not target.is_file():
            continue
        for declaration, _ in read_page(target).headers:
            match = re.fullmatch(r'pub type (\w+) = (bool|[ui](?:8|16|32|64|128|size)|f(?:32|64));', declaration)
            if match:
                aliases[match[1]] = match[2]
    return aliases


def resolve_reexport_owners(data, root, documentation_root=None):
    """Resolve omitted rustdoc source links through explicit public Rust imports.

    Args:
        data: Fresh compiler inventory from every canonical direction/API root.
        root: Source checkout containing the actual public re-export declarations.
        documentation_root: Optional freshly built canonical rustdoc for explicit imported modules.
    Returns:
        None; fills missing owners only after exact import/type/signature agreement.
    Raises:
        ValueError when an owner cannot be proven. Identical type names alone
        never establish identity across independent PHY crates.
    """
    from implementation_evidence import block, rust_code
    exports = {item['export']: item for items in data.values() for item in items}
    for item in exports.values():
        missing = [method for method in item['methods'] if method['source_file'] is None]
        if not missing:
            continue
        parts = item['export'].split('::')
        owner = SOURCE_CRATES.get(parts[0])
        if owner is None:
            raise ValueError(f"No source crate for public re-export: {item['export']}")
        directory = root / owner / 'src'
        module = directory.joinpath(*parts[1:-1])
        paths = [module.with_suffix('.rs'), module / 'mod.rs'] if len(parts) > 2 else [directory / 'lib.rs']
        source = next((path for path in paths if path.is_file()), None)
        inline_facade = parts[:-1] == ['merlin', 'common'] and source is None
        code = rust_code(source.read_text()) if source is not None else ''
        if inline_facade and (directory / 'lib.rs').is_file():
            # The facade's common namespace is declared inline. Only its exact
            # top-level public module can establish these imports; another
            # module containing the same type name is not source evidence.
            facade = rust_code((directory / 'lib.rs').read_text())
            # Find each whole public inline module opener, e.g. `pub mod common {`.
            # No groups/flags; literal-masked code and brace depth exclude nested
            # modules, comments and string decoys. Empty/multiple bodies prove none.
            bodies = [block(facade, match.end() - 1)
                      for match in re.finditer(r'\bpub\s+mod\s+common\s*\{', facade)
                      if facade[:match.start()].count('{') == facade[:match.start()].count('}')]
            if len(bodies) == 1:
                code = bodies[0]
        origins = set()
        if code:
            # Explicit grouped pub use, e.g. docsis_ofdma::api::{ApiError, ApiResult}.
            # Groups are full source namespace and comma-delimited names/aliases.
            for match in re.finditer(r'\bpub\s+use\s+([\w:]+)::\{([^{}]+)\}\s*;', code):
                if (inline_facade
                        and code[:match.start()].count('{') != code[:match.start()].count('}')):
                    continue
                # merlin_api normalizes to this very facade namespace. Resolve
                # the facade's origin through the actual compiler crate index,
                # never through its own missing-source inventory copy.
                namespace = match[1] if inline_facade else API_NAMESPACES.get(match[1], match[1])
                if match[1].split('::')[0] not in SOURCE_CRATES:
                    continue
                for entry in match[2].split(','):
                    names = entry.strip().split(' as ')
                    if names[-1].strip() == parts[-1]:
                        origins.add(namespace + '::' + names[0].strip())
        if len(origins) != 1:
            raise ValueError(f"No unique explicit source for re-export: {item['export']}")
        origin = origins.pop()
        original = exports.get(origin)
        if original is None and documentation_root is not None:
            namespace = origin.rsplit('::', 1)[0]
            index = Path(documentation_root).joinpath(*namespace.split('::'), 'index.html')
            if index.is_file():
                candidates = [value for value in inventory(namespace, index, Path(documentation_root))
                              if value['export'] == origin]
                if len(candidates) == 1:
                    original = candidates[0]
        if original is None or original['kind'] != item['kind']:
            raise ValueError(f"Re-export kind/identity is not in compiler inventory: {item['export']}")
        for method in missing:
            candidates = []
            for value in original['methods']:
                if (value['operation'].rsplit('::', 1)[-1] != method['operation'].rsplit('::', 1)[-1]
                        or value['source_file'] is None):
                    continue
                signature = value['signature']
                if signature != method['signature']:
                    aliases = compiler_primitive_aliases(value, documentation_root)
                    for name, primitive in aliases.items():
                        signature = re.sub(r'\b' + re.escape(name) + r'\b', primitive, signature)
                if signature == method['signature']:
                    candidates.append((value, False))
                    continue
                # The exact import and compiler item kind above establish one
                # nominal type. Rustdoc may spell that type as Self, its original
                # name or its exposed alias. Preserve every other signature token.
                # Whole-token substitutions, no flags/groups: Result<Self, E>
                # and Result<Project, E> agree for the proven Project import;
                # Selfish and elsewhere::Project remain distinct signatures.
                original_name = original['export'].rsplit('::', 1)[-1]
                canonical = re.sub(r'\bSelf\b', original_name, signature)
                exposed = re.sub(r'\bSelf\b', original_name, method['signature'])
                exposed = re.sub(r'\b' + re.escape(parts[-1]) + r'\b', original_name, exposed)
                if canonical == exposed:
                    candidates.append((value, True))
            if len(candidates) != 1:
                raise ValueError(f"Re-export method has no exact canonical owner: {method['operation']}")
            canonical, nominal_spelling = candidates[0]
            method['source_file'] = canonical['source_file']
            method['reexport_origin'] = original['export']
            if nominal_spelling:
                method['reexport_canonical_signature'] = canonical['signature']


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
        "common": ("merlin::common", "merlin/common/index.html"),
        "native-input": ("pnm_input", "pnm_input/index.html"),
    }
    data = {name: inventory(namespace, args.docs / path, args.docs)
            for name, (namespace, path) in roots.items()}
    resolve_reexport_owners(data, Path(__file__).resolve().parents[2], args.docs)
    args.output.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
    print("Rustdoc exports:", {name: sum(len(item["methods"]) for item in items)
                              for name, items in data.items()})


if __name__ == "__main__":
    main()
