{
  name: 'boolean',
  levels: [
    import 'if.jsonnet',
    import 'not.jsonnet',
    import 'and.jsonnet',
    import 'or.jsonnet',
    import 'xor.jsonnet',
  ],
  section_constants: [
    ['TRUE', 'a:b: a'],
    ['FALSE', 'a:b: b'],
  ],
}
