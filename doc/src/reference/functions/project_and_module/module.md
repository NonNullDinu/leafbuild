# The `module()` function

## Parameters
```leafbuild
module (
        module_name,
        [languages: languages],
        [authors: authors],
        [maintainers: maintainers]
)
```

## Description
Tells the build system that the folder we are in should be treated
as a module.

## Returns
This function doesn't return anything and is allowed only as a
standalone function call(Not allowed in expressions).

## Positional parameters

### module_name
> **Type**: `string`

The name of the module.

## Kw parameters

### languages (optional)
> **Type**: `string` or array of `string`s, that are valid languages;
> see [this](../../../supported_languages.md) for the list of supported
> languages and how to configure them.

#### Description

The language or languages required to build this module.

#### Default value
The default value is taken from the parent module/project.

### authors (optional)
> **Type**: `author` or array of `author`s; see
> [the author type](../../special_types/author.md)
> on how to build an author object.

#### Description

The list of authors of the module.

#### Default value
The default value is taken from the parent module/project.

### maintainers (optional)

> **Type**: `maintainer` or array of `maintainer`s; see
> [the maintainer type](../../special_types/maintainer.md)
> on how to build a maintainer object.

#### Description
The list of maintainers of the module.

#### Default value
The default value is taken from the parent module/project.

## Aliases

`mod()`