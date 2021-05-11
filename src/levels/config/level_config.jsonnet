{
  sections: [
    import 'basic/section.jsonnet',
    import 'boolean/section.jsonnet',
    import 'pair_and_list/section.jsonnet',
    import 'recursion/section.jsonnet',
    import 'numerals/section.jsonnet',
  ],
  tests: (import 'boolean/tests.jsonnet') + (import 'pair_and_list/tests.jsonnet') + (import 'numerals/tests.jsonnet'),
}
