/**
 * @file A schema for protocol creation and an implementation-agnostic design language
 * @author warpfork
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "dapper",

  rules: {
    source_file: ($) =>
      repeat(
        choice(
          $.type_declaration,
          // TODO: other kinds of definitions
        ),
      ),

    type_declaration: ($) =>
      seq(
        "type",
        field("name", $.type_identifier),
        choice(
          $.type_defn_unit,
          $.type_defn_bool,
          $.type_defn_int,
          $.type_defn_float,
          $.type_defn_string,
          $.type_defn_bytes,
          $.type_defn_list,
          $.type_defn_map,
          $.type_defn_link,
          $.type_defn_struct,
          $.type_defn_enum,
          $.type_defn_variant,
        ),
      ),

    type_defn_unit: ($) => seq("unit", $.representation_clause_unit),
    type_defn_bool: ($) => "bool",
    type_defn_int: ($) => "int", // TODO: constraints extension
    type_defn_float: ($) => "float", // TODO: constraints extension
    type_defn_string: ($) => "string", // TODO: constraints extension
    type_defn_bytes: ($) =>
      seq("bytes", optional($.representation_clause_bytes)),
    type_defn_list: ($) => seq("list", $.type_clause_list),
    type_defn_map: ($) =>
      seq("map", $.type_clause_map, optional($.representation_clause_map)),
    type_defn_link: ($) => seq("link", $.type_clause_link), // TODO: decide if these deserve representation clauses.  Probably yes: base encodings can go here.
    type_defn_struct: ($) =>
      seq("struct", $.type_clause_struct, $.representation_clause_struct),
    type_defn_enum: ($) =>
      seq("enum", $.type_clause_enum, $.representation_clause_enum), // FIXME: I suspect this is actually a fused choice inside; strings and int representations ramify back up to the type body.  But we could leave that to the logical validation stage; giving syntax highlight to something that's not gonna validate isn't the worst here.
    type_defn_variant: ($) =>
      seq("variant", $.type_clause_variant, $.representation_clause_variant),

    type_identifier: ($) => /[A-Z][A-Za-z0-9_]*/,

    representation_clause_unit: ($) =>
      seq(
        "representation",
        choice("null", "true", "false", "emptymap", "emptylist"),
      ),

    representation_clause_bytes: ($) =>
      seq("representation", choice("base64", "base58", "base32", "base16")),

    // List type bodies can be found in a named type declaration (after the "list" keyword),
    // in which case they may also be followed by a representation clause;
    // or, they may be found inline (in fields, in other lists or maps),
    // in which case their representation cannot be defined (one should use a named type for that much complexity!).
    // ... JK: lists don't actually have representation clauses anyway.
    type_clause_list: ($) => seq("[", $.type_or_reference, "]"),

    // Some things about map declarations are constrainted at the AST level
    // (namely, map keys _must_ be named types),
    // but many other things aren't (or can't be) checked at the AST level:
    // whether the key type has a string representation, in particular.
    type_clause_map: ($) =>
      seq("{", $.type_identifier, ":", $.type_or_reference, "}"),

    representation_clause_map: ($) =>
      seq(
        "representation",
        choice(
          // The default would be "map", but if you mean that, just don't say anything.
          seq("stringpairs"), // TODO: this has parameters
          "listpairs",
        ),
      ),

    type_clause_link: ($) => seq("&", $.type_identifier),

    type_clause_struct: ($) =>
      seq(
        "{",
        repeat(
          seq(
            $.field_identifier,
            optional("optional"),
            $.type_or_reference,
            optional(seq("(", ")")), // TODO: mega-todo in here.  defaults; field renames; other things I can't recall.  Many only relevant to certain representations, which is a bit gnarly.
          ),
        ),
        "}",
      ),
    field_identifier: ($) => /[A-Za-z0-9_]*/,

    representation_clause_struct: ($) =>
      seq(
        "representation",
        choice(
          "map", // We make you say this for structs, rather than having any default at all.  Explicit is good sometimes.  Up for review, though.
          "tuple",
          seq("stringpairs"), // TODO: this has parameters
          seq("stringjoin"), // TODO: this has parameters
          "listpairs",
        ),
      ),

    type_clause_enum: ($) => seq("TODO"),
    representation_clause_enum: ($) => seq("TODO"),
    type_clause_variant: ($) => seq("TODO"),
    representation_clause_variant: ($) => seq("TODO"),

    // type_or_reference is either an inline type definition or the name of a type.
    type_or_reference: ($) =>
      choice($.type_identifier, $.type_definition_inline),

    type_definition_inline: ($) =>
      choice(
        $.type_clause_list,
        $.type_clause_map,
        $.type_clause_variant, // this one's complicated, especially thrusting fun times upon the codegen for some of the more limited languages out there, but it's still just so valuable from a dev gtd perspective.
        $.type_clause_link,
      ),
  },
});
