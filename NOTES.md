`AttributeInterceptor` is needed to forward attributes to the correct place. They usually need to live at the very top-level of the `view!` in order for them to catch attributes. Exceptions to this rule is:

- `Provider`: Attributes correctly get caught by an `AttributeInterceptor` nested inside a `Provider` as in `Checkbox`

