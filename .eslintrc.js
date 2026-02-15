module.exports = {
  root: true,
  extends: ['next/core-web-vitals'],
  parserOptions: {
    project: './frontend/tsconfig.json',
  },
  rules: {
    'no-console': ['warn', { allow: ['warn', 'error'] }],
    'max-lines': ['warn', { max: 300, skipBlankLines: true, skipComments: true }],
    'max-lines-per-function': ['warn', 50]
  },
  overrides: [
    {
      files: ['*.ts', '*.tsx'],
      rules: {
        '@typescript-eslint/naming-convention': [
          'error',
          { selector: 'variableLike', format: ['camelCase', 'UPPER_CASE'] },
          { selector: 'typeLike', format: ['PascalCase'] }
        ]
      }
    },
    {
      files: ['*.test.*', '*.spec.*'],
      env: { jest: true }
    }
  ]
};
